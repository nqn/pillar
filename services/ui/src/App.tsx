import { useState, useEffect, useMemo, Fragment } from 'react'
import { useLocalStorage } from './hooks/useLocalStorage'
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'

import {
    ArchiveIcon,
    CalendarIcon,
    ChevronRightIcon,
    ChevronDownIcon,
    MixerHorizontalIcon,
    UpdateIcon,
    ExclamationTriangleIcon,
    Cross2Icon,
    EnterFullScreenIcon,
    ExitFullScreenIcon,
    MoonIcon,
    SunIcon,
    CheckCircledIcon,
    DiscIcon,
    ClockIcon,
    HeightIcon,
    LayersIcon,
    CubeIcon,
    DoubleArrowLeftIcon,
    HamburgerMenuIcon,
    ViewGridIcon,
    ListBulletIcon,
    PlusIcon,
    Pencil1Icon,
    CheckIcon,
} from '@radix-ui/react-icons'
import { MarkdownEditor } from './components/MarkdownEditor'
import { ScrollArea } from './components/ui/ScrollArea'
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from './components/ui/Select'
import { SearchInput } from './components/ui/SearchInput'
import {
    DropdownMenu,
    DropdownMenuCheckboxItem,
    DropdownMenuItem,
    DropdownMenuContent,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from './components/ui/DropdownMenu'

// Types
interface Project {
    id: string
    name: string
    status: string
    priority: string
    description?: string
}

interface Milestone {
    id: string
    title: string
    status: string
    project: string
    target_date?: string
    created?: string
}

interface Issue {
    id: string
    number: string
    title: string
    status: string
    priority: string
    project: string
    milestone?: string
    description?: string
}

const statusWeight: Record<string, number> = {
    'backlog': 1,
    'todo': 2,
    'in-progress': 3,
    'completed': 4,
    'cancelled': 0
}

const priorityWeight: Record<string, number> = {
    'low': 1,
    'medium': 2,
    'high': 3,
    'urgent': 4
}

const StatusIcon = ({ status, size = 16 }: { status: string, size?: number }) => {
    switch (status) {
        case 'completed': return <CheckCircledIcon style={{ width: size, height: size }} className="status-completed" />
        case 'in-progress': return <ClockIcon style={{ width: size, height: size }} className="status-progress" />
        case 'cancelled': return <Cross2Icon style={{ width: size, height: size }} className="status-cancelled" />
        default: return <DiscIcon style={{ width: size, height: size }} className="status-todo" />
    }
}

const PriorityDot = ({ priority }: { priority: string }) => {
    return <span className={`priority-badge-dot priority-${priority}`} />
}

const Modal = ({ title, children, onConfirm, onCancel }: any) => (
    <div className="modal-overlay">
        <div className="modal-content">
            <div className="flex justify-between items-center mb-6">
                <h2 style={{ fontSize: '18px', fontWeight: 600 }}>{title}</h2>
                <button className="btn-icon" onClick={onCancel}><Cross2Icon /></button>
            </div>
            <div className="modal-body">
                {children}
            </div>
            <div className="form-actions">
                <button className="btn" onClick={onCancel}>Cancel</button>
                <button className="btn btn-primary" onClick={onConfirm}>Create</button>
            </div>
        </div>
    </div>
)

function App() {
    const [theme, setTheme] = useLocalStorage<'dark' | 'light'>('pillar-theme', 'dark')
    const [projects, setProjects] = useState<Project[]>([])
    const [milestones, setMilestones] = useState<Milestone[]>([])
    const [issues, setIssues] = useState<Issue[]>([])
    const [loading, setLoading] = useState(true)

    const [selectedProjectId, setSelectedProjectId] = useState<string | null>(null)
    const [selectedMilestoneId, setSelectedMilestoneId] = useState<string | null>(null)
    const [selectedIssue, setSelectedIssue] = useState<Issue | null>(null)

    const [statusFilter, setStatusFilter] = useLocalStorage<string[]>('pillar-status-filter', [])
    const [priorityFilter, setPriorityFilter] = useLocalStorage<string[]>('pillar-priority-filter', [])
    const [searchTerm, setSearchTerm] = useState('')
    const [currentView, setCurrentView] = useState<'issues' | 'projects'>('issues')
    const [isDetailMaximized, setIsDetailMaximized] = useLocalStorage('pillar-detail-maximized', false)
    const [isSidebarCollapsed, setIsSidebarCollapsed] = useLocalStorage('pillar-sidebar-collapsed', false)
    const [collapsedGroupsArray, setCollapsedGroupsArray] = useLocalStorage<string[]>('pillar-collapsed-groups', [])
    const collapsedGroups = useMemo(() => new Set(collapsedGroupsArray), [collapsedGroupsArray])
    const [projectViewMode, setProjectViewMode] = useLocalStorage<'grid' | 'list'>('pillar-project-view-mode', 'grid')

    const toggleGroup = (label: string) => {
        setCollapsedGroupsArray(prev => {
            const next = new Set(prev)
            if (next.has(label)) next.delete(label)
            else next.add(label)
            return Array.from(next)
        })
    }

    const [sortBy, setSortBy] = useLocalStorage<string>('pillar-sort-by', 'number')
    const [groupBy, setGroupBy] = useLocalStorage<string>('pillar-group-by', 'none')
    const [sidebarProjectStatus, setSidebarProjectStatus] = useLocalStorage<'all' | 'active' | 'completed'>('pillar-sidebar-project-status', 'active')

    useEffect(() => {
        document.documentElement.setAttribute('data-theme', theme)
    }, [theme])

    const [isEditing, setIsEditing] = useState(false)
    const [editedDescription, setEditedDescription] = useState('')
    const [isCreateModalOpen, setIsCreateModalOpen] = useState<'project' | 'milestone' | 'issue' | null>(null)
    const [createFormData, setCreateFormData] = useState({
        name: '',
        id: '',
        title: '',
        project: '',
        priority: 'medium',
        milestone: '',
        date: '',
        tags: ''
    })

    const fetchData = async () => {
        setLoading(true)
        try {
            const res = await fetch('/api/data')
            const data = await res.json()
            setProjects(data.projects || [])
            setMilestones(data.milestones || [])
            setIssues(data.issues || [])
        } catch (error) {
            console.error('Failed to fetch data:', error)
        } finally {
            setLoading(false)
        }
    }

    useEffect(() => {
        fetchData()
    }, [])

    const processedIssues = useMemo(() => {
        // 1. Filter
        let result = issues.filter(issue => {
            if (selectedProjectId && issue.project !== selectedProjectId) return false
            if (selectedMilestoneId && issue.milestone !== selectedMilestoneId) return false
            if (statusFilter.length > 0 && !statusFilter.includes(issue.status)) return false
            if (priorityFilter.length > 0 && !priorityFilter.includes(issue.priority)) return false
            if (searchTerm && !issue.title.toLowerCase().includes(searchTerm.toLowerCase())) return false
            return true
        })

        // 2. Sort
        result.sort((a, b) => {
            switch (sortBy) {
                case 'priority':
                    return (priorityWeight[b.priority] || 0) - (priorityWeight[a.priority] || 0)
                case 'status':
                    return (statusWeight[b.status] || 0) - (statusWeight[a.status] || 0)
                case 'title':
                    return a.title.localeCompare(b.title)
                case 'number':
                default:
                    return b.number.localeCompare(a.number)
            }
        })

        return result
    }, [issues, selectedProjectId, selectedMilestoneId, statusFilter, priorityFilter, searchTerm, sortBy])

    const processedProjects = useMemo(() => {
        let result = projects.filter(p => {
            if (statusFilter.length > 0 && !statusFilter.includes(p.status)) return false
            if (priorityFilter.length > 0 && !priorityFilter.includes(p.priority)) return false
            if (searchTerm && !p.name.toLowerCase().includes(searchTerm.toLowerCase()) && !p.id.toLowerCase().includes(searchTerm.toLowerCase())) return false
            return true
        })

        result.sort((a, b) => {
            if (sortBy === 'priority') {
                return (priorityWeight[b.priority] || 0) - (priorityWeight[a.priority] || 0)
            }
            if (sortBy === 'status') {
                return (statusWeight[b.status] || 0) - (statusWeight[a.status] || 0)
            }
            return a.name.localeCompare(b.name)
        })

        return result
    }, [projects, searchTerm, statusFilter, priorityFilter, sortBy])

    const groupedProjects = useMemo(() => {
        if (groupBy === 'none') {
            return [{ label: '', projects: processedProjects }]
        }
        const groups: Record<string, Project[]> = {}
        processedProjects.forEach(p => {
            let key = 'Other'
            if (groupBy === 'status') key = p.status
            if (groupBy === 'priority') key = p.priority
            if (!groups[key]) groups[key] = []
            groups[key].push(p)
        })
        const labels = Object.keys(groups).sort()
        return labels.map(label => ({ label, projects: groups[label] }))
    }, [processedProjects, groupBy])

    const groupedIssues = useMemo(() => {
        if (groupBy === 'none') {
            return [{ label: '', issues: processedIssues }]
        }

        const groups: Record<string, Issue[]> = {}

        processedIssues.forEach(issue => {
            let key = 'Other'
            if (groupBy === 'status') key = issue.status
            if (groupBy === 'priority') key = issue.priority
            if (groupBy === 'project') key = issue.project
            if (groupBy === 'milestone') key = issue.milestone || 'No Milestone'

            if (!groups[key]) groups[key] = []
            groups[key].push(issue)
        })

        // Sort groups by key weight if applicable
        const groupLabels = Object.keys(groups)
        if (groupBy === 'status') {
            groupLabels.sort((a, b) => (statusWeight[b] || 0) - (statusWeight[a] || 0))
        } else if (groupBy === 'priority') {
            groupLabels.sort((a, b) => (priorityWeight[b] || 0) - (priorityWeight[a] || 0))
        } else {
            groupLabels.sort()
        }

        return groupLabels.map(label => ({
            label,
            issues: groups[label]
        }))
    }, [processedIssues, groupBy])

    const toggleTheme = () => setTheme(theme === 'dark' ? 'light' : 'dark')

    const handleSaveIssue = async () => {
        if (!selectedIssue) return
        try {
            const [proj, num] = selectedIssue.id.split('/')
            await fetch(`/api/issues/${proj}/${num}`, {
                method: 'PATCH',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    status: selectedIssue.status,
                    priority: selectedIssue.priority,
                    description: editedDescription
                })
            })
            setIsEditing(false)
            fetchData()
        } catch (error) {
            console.error('Failed to save issue:', error)
        }
    }

    const handleSaveProject = async () => {
        if (!selectedProjectId) return
        try {
            await fetch(`/api/projects/${selectedProjectId}`, {
                method: 'PATCH',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    description: editedDescription
                })
            })
            setIsEditing(false)
            fetchData()
        } catch (error) {
            console.error('Failed to save project:', error)
        }
    }

    const handleCreate = async () => {
        let url = ''
        let body: any = {}
        if (isCreateModalOpen === 'project') {
            url = '/api/projects'
            body = {
                name: createFormData.name,
                id: createFormData.id || undefined,
                priority: createFormData.priority
            }
        } else if (isCreateModalOpen === 'issue') {
            url = '/api/issues'
            body = {
                project: createFormData.project || selectedProjectId,
                title: createFormData.title,
                priority: createFormData.priority,
                milestone: createFormData.milestone === '_none' ? undefined : (createFormData.milestone || undefined),
                tags: createFormData.tags || undefined
            }
        } else if (isCreateModalOpen === 'milestone') {
            url = '/api/milestones'
            body = {
                project: createFormData.project || selectedProjectId,
                title: createFormData.title,
                date: createFormData.date || undefined
            }
        }

        try {
            const res = await fetch(url, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(body)
            })
            if (res.ok) {
                setIsCreateModalOpen(null)
                setCreateFormData({
                    name: '', id: '', title: '', project: '', priority: 'medium', milestone: '', date: '', tags: ''
                })
                fetchData()
            } else {
                const error = await res.text()
                alert(`Error: ${error}`)
            }
        } catch (error) {
            console.error('Failed to create:', error)
        }
    }


    return (
        <div className={`app-container ${isSidebarCollapsed ? 'sidebar-collapsed' : ''}`}>
            {/* Sidebar */}
            <aside className="sidebar">
                <div className="sidebar-header">
                    <div className="logo">PILLAR</div>
                    <button
                        className="btn-icon sidebar-toggle"
                        onClick={() => setIsSidebarCollapsed(true)}
                        title="Collapse sidebar"
                    >
                        <DoubleArrowLeftIcon />
                    </button>
                </div>
                <ScrollArea className="sidebar-content">
                    <div className="nav-section">
                        <div className="nav-section-title">Workspace</div>
                        <div
                            className={`nav-item ${currentView === 'issues' && selectedProjectId === null ? 'active' : ''}`}
                            onClick={() => { setCurrentView('issues'); setSelectedProjectId(null); setSelectedMilestoneId(null); }}
                            title="All Issues"
                        >
                            <ArchiveIcon />
                            {!isSidebarCollapsed && <span>All Issues</span>}
                        </div>
                        <div
                            className={`nav-item ${currentView === 'projects' && selectedProjectId === null ? 'active' : ''}`}
                            onClick={() => { setCurrentView('projects'); setSelectedProjectId(null); }}
                            title="All Projects"
                        >
                            <CubeIcon />
                            {!isSidebarCollapsed && <span>All Projects</span>}
                        </div>
                    </div>

                    <div className="nav-section">
                        <div className="nav-section-title" style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                            <span>Projects</span>
                            {!isSidebarCollapsed && (
                                <div className="flex gap-1">
                                    <button
                                        className={`sidebar-filter-btn ${sidebarProjectStatus === 'active' ? 'active' : ''}`}
                                        onClick={(e) => { e.stopPropagation(); setSidebarProjectStatus('active'); }}
                                    >Active</button>
                                    <button
                                        className={`sidebar-filter-btn ${sidebarProjectStatus === 'completed' ? 'active' : ''}`}
                                        onClick={(e) => { e.stopPropagation(); setSidebarProjectStatus('completed'); }}
                                    >Done</button>
                                    <button
                                        className={`sidebar-filter-btn ${sidebarProjectStatus === 'all' ? 'active' : ''}`}
                                        onClick={(e) => { e.stopPropagation(); setSidebarProjectStatus('all'); }}
                                    >All</button>
                                </div>
                            )}
                        </div>
                        {projects.filter(p => {
                            if (sidebarProjectStatus === 'all') return true
                            if (sidebarProjectStatus === 'active') return p.status !== 'completed' && p.status !== 'cancelled'
                            if (sidebarProjectStatus === 'completed') return p.status === 'completed' || p.status === 'cancelled'
                            return true
                        }).map(project => (
                            <div
                                key={project.id}
                                className={`nav-item ${selectedProjectId === project.id ? 'active' : ''}`}
                                onClick={() => {
                                    setSelectedProjectId(project.id);
                                    setCurrentView('projects');
                                    setSelectedMilestoneId(null);
                                }}
                                title={project.name}
                                style={{ justifyContent: 'space-between' }}
                            >
                                <div className="flex items-center gap-2 overflow-hidden">
                                    <StatusIcon status={project.status} size={12} />
                                    {!isSidebarCollapsed && <span className="truncate">{project.name}</span>}
                                </div>
                                {!isSidebarCollapsed && <PriorityDot priority={project.priority} />}
                            </div>
                        ))}
                        {!isSidebarCollapsed && (
                            <div 
                                className="nav-item text-subtle" 
                                style={{ marginTop: '8px', opacity: 0.7, border: '1px dashed var(--border-muted)', justifyContent: 'center' }}
                                onClick={() => { setIsCreateModalOpen('project'); setCreateFormData(prev => ({ ...prev, project: '' })); }}
                            >
                                <PlusIcon /> <span>New Project</span>
                            </div>
                        )}
                    </div>

                    {selectedProjectId && (
                        <div className="nav-section">
                            <div className="nav-section-title">Milestones</div>
                            {milestones.filter(m => m.project === selectedProjectId).map(milestone => (
                                <div
                                    key={milestone.id}
                                    className={`nav-item ${selectedMilestoneId === milestone.id ? 'active' : ''}`}
                                    onClick={() => setSelectedMilestoneId(milestone.id)}
                                    title={milestone.title}
                                >
                                    <CalendarIcon />
                                    {!isSidebarCollapsed && <span>{milestone.title}</span>}
                                </div>
                            ))}
                            {!isSidebarCollapsed && (
                                <div 
                                    className="nav-item text-subtle" 
                                    style={{ marginTop: '4px', opacity: 0.7, fontSize: '12px' }}
                                    onClick={() => { setIsCreateModalOpen('milestone'); setCreateFormData(prev => ({ ...prev, project: selectedProjectId })); }}
                                >
                                    <PlusIcon /> <span>Add Milestone</span>
                                </div>
                            )}
                        </div>
                    )}
                </ScrollArea>

                <div className="sidebar-footer" style={{ padding: '8px', borderTop: '1px solid var(--border-muted)' }}>
                    <div className="nav-item" onClick={toggleTheme} title={theme === 'dark' ? 'Light Mode' : 'Dark Mode'}>
                        {theme === 'dark' ? <SunIcon /> : <MoonIcon />}
                        {!isSidebarCollapsed && <span>{theme === 'dark' ? 'Light Mode' : 'Dark Mode'}</span>}
                    </div>
                </div>
            </aside>

            <main className="main-content">
                <header className="header">
                    <div className="header-left">
                        {isSidebarCollapsed && (
                            <button
                                className="btn-icon sidebar-expand-button"
                                onClick={() => setIsSidebarCollapsed(false)}
                                title="Expand sidebar"
                            >
                                <HamburgerMenuIcon />
                            </button>
                        )}
                        <div className="breadcrumb">
                            <span className="fg-subtle">Workspace</span>
                            {selectedProjectId && (
                                <>
                                    <span className="sep"><ChevronRightIcon /></span>
                                    <span>{projects.find(p => p.id === selectedProjectId)?.name}</span>
                                </>
                            )}
                            {selectedMilestoneId && (
                                <>
                                    <span className="sep"><ChevronRightIcon /></span>
                                    <span className="fg-subtle">{milestones.find(m => m.id === selectedMilestoneId)?.title}</span>
                                </>
                            )}
                        </div>
                    </div>
                </header>

                {/* Filter Bar */}
                <div className="filter-bar">
                    <div className="filter-group">
                        <MixerHorizontalIcon className="text-subtle" />

                        <DropdownMenu>
                            <DropdownMenuTrigger asChild>
                                <button className="ui-select-trigger">
                                    <span className="flex items-center gap-2">
                                        <MixerHorizontalIcon className="text-subtle" style={{ width: 12, height: 12 }} />
                                        {statusFilter.length === 0 ? 'Any Status' :
                                            statusFilter.length === 1 ? statusFilter[0] :
                                                `${statusFilter.length} statuses`}
                                    </span>
                                    <ChevronDownIcon className="ui-select-icon" />
                                </button>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent>
                                <DropdownMenuLabel>Status</DropdownMenuLabel>
                                <DropdownMenuSeparator />
                                {['backlog', 'todo', 'in-progress', 'completed', 'cancelled'].map(status => (
                                    <DropdownMenuCheckboxItem
                                        key={status}
                                        checked={statusFilter.includes(status)}
                                        onCheckedChange={(checked) => {
                                            setStatusFilter(prev =>
                                                checked ? [...prev, status] : prev.filter(s => s !== status)
                                            )
                                        }}
                                        className="capitalize"
                                    >
                                        {status.replace('-', ' ')}
                                    </DropdownMenuCheckboxItem>
                                ))}
                                {statusFilter.length > 0 && (
                                    <>
                                        <DropdownMenuSeparator />
                                        <DropdownMenuItem onClick={() => setStatusFilter([])}>
                                            Clear Filters
                                        </DropdownMenuItem>
                                    </>
                                )}
                            </DropdownMenuContent>
                        </DropdownMenu>

                        <div className="divider-v" />

                        <DropdownMenu>
                            <DropdownMenuTrigger asChild>
                                <button className="ui-select-trigger">
                                    <span className="flex items-center gap-2">
                                        <MixerHorizontalIcon className="text-subtle" style={{ width: 12, height: 12 }} />
                                        {priorityFilter.length === 0 ? 'Any Priority' :
                                            priorityFilter.length === 1 ? priorityFilter[0] :
                                                `${priorityFilter.length} priorities`}
                                    </span>
                                    <ChevronDownIcon className="ui-select-icon" />
                                </button>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent>
                                <DropdownMenuLabel>Priority</DropdownMenuLabel>
                                <DropdownMenuSeparator />
                                {['low', 'medium', 'high', 'urgent'].map(priority => (
                                    <DropdownMenuCheckboxItem
                                        key={priority}
                                        checked={priorityFilter.includes(priority)}
                                        onCheckedChange={(checked) => {
                                            setPriorityFilter(prev =>
                                                checked ? [...prev, priority] : prev.filter(p => p !== priority)
                                            )
                                        }}
                                        className="capitalize"
                                    >
                                        {priority}
                                    </DropdownMenuCheckboxItem>
                                ))}
                                {priorityFilter.length > 0 && (
                                    <>
                                        <DropdownMenuSeparator />
                                        <DropdownMenuItem onClick={() => setPriorityFilter([])}>
                                            Clear Filters
                                        </DropdownMenuItem>
                                    </>
                                )}
                            </DropdownMenuContent>
                        </DropdownMenu>

                        <div className="divider-v" />

                        <HeightIcon className="text-subtle" />
                        <Select value={sortBy} onValueChange={setSortBy}>
                            <SelectTrigger>
                                <SelectValue placeholder="Sort" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="number">Number</SelectItem>
                                <SelectItem value="priority">Priority</SelectItem>
                                <SelectItem value="status">Status</SelectItem>
                                <SelectItem value="title">Title</SelectItem>
                            </SelectContent>
                        </Select>

                        <div className="divider-v" />

                        <LayersIcon className="text-subtle" />
                        <Select value={groupBy} onValueChange={setGroupBy}>
                            <SelectTrigger>
                                <SelectValue placeholder="Group" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="none">No Grouping</SelectItem>
                                <SelectItem value="status">Status</SelectItem>
                                <SelectItem value="priority">Priority</SelectItem>
                                <SelectItem value="project">Project</SelectItem>
                                <SelectItem value="milestone">Milestone</SelectItem>
                            </SelectContent>
                        </Select>

                        <div className="divider-v" />

                        <div style={{ width: '220px' }}>
                            <SearchInput
                                placeholder="Search issues..."
                                value={searchTerm}
                                onChange={(e) => setSearchTerm(e.target.value)}
                            />
                        </div>

                        <button 
                            className="btn btn-primary" 
                            style={{ height: '32px', padding: '0 12px' }}
                            onClick={() => { setIsCreateModalOpen('issue'); setCreateFormData(prev => ({ ...prev, project: selectedProjectId || '' })); }}
                        >
                            <PlusIcon /> {!isSidebarCollapsed && 'New Issue'}
                        </button>
                    </div>
                    <div className="filter-results">
                        {processedIssues.length} issues
                    </div>
                </div>

                {currentView === 'issues' ? (
                    <div className="view-pane-container">
                        {/* Issue List View */}
                        <div className="flex-1 overflow-y-auto">
                            <div className="issue-table">
                                {loading ? (
                                    <div className="loading-overlay">
                                        <UpdateIcon className="animate-spin" />
                                        <span>Loading...</span>
                                    </div>
                                ) : processedIssues.length === 0 ? (
                                    <div className="empty-state">
                                        <ExclamationTriangleIcon style={{ width: 32, height: 32, marginBottom: '1rem', opacity: 0.5 }} />
                                        <h3>No items match your filters</h3>
                                        <p>Try clearing your status or priority filters.</p>
                                    </div>
                                ) : (
                                    <div className="issue-list-content">
                                        {groupedIssues.map((group, groupIdx) => {
                                            const isCollapsed = collapsedGroups.has(group.label)
                                            return (
                                                <div key={groupIdx} className={`issue-group ${isCollapsed ? 'is-collapsed' : ''}`}>
                                                    {group.label && (
                                                        <div className="issue-group-header clickable" onClick={() => toggleGroup(group.label)}>
                                                            <div className="flex items-center gap-2">
                                                                <ChevronRightIcon
                                                                    className={`group-chevron ${isCollapsed ? '' : 'rotate-90'}`}
                                                                />
                                                                {group.label}
                                                            </div>
                                                            <span className="group-count">{group.issues.length}</span>
                                                        </div>
                                                    )}
                                                    {!isCollapsed && group.issues.map((issue: Issue) => (
                                                        <div
                                                            key={issue.id}
                                                            className={`issue-row ${selectedIssue?.id === issue.id ? 'selected' : ''}`}
                                                            onClick={() => setSelectedIssue(issue)}
                                                        >
                                                            <div className="issue-cell-status">
                                                                <StatusIcon status={issue.status} />
                                                            </div>
                                                            <div className="issue-cell-id">#{issue.number}</div>
                                                            <div className="issue-cell-title">
                                                                {issue.title}
                                                                {selectedProjectId === null && (
                                                                    <span className="project-tag-hint">
                                                                        {issue.project}
                                                                    </span>
                                                                )}
                                                            </div>
                                                            <div style={{ width: '90px', display: 'flex', alignItems: 'center', gap: '8px' }}>
                                                                <PriorityDot priority={issue.priority} />
                                                                <span className="priority-label" style={{ fontSize: '11px', textTransform: 'capitalize' }}>
                                                                    {issue.priority}
                                                                </span>
                                                            </div>
                                                        </div>
                                                    ))}
                                                </div>
                                            )
                                        })}
                                    </div>
                                )}
                            </div>
                        </div>

                        {/* Issue Detail Panel */}
                        {selectedIssue && (
                            <aside className={`detail-panel ${isDetailMaximized ? 'maximized' : ''}`}>
                                <header className="detail-header">
                                    <div className="detail-actions">
                                        <span className="issue-number">#{selectedIssue.number}</span>
                                        <div className="flex items-center gap-2">
                                            {isEditing ? (
                                                <>
                                                    <button className="btn btn-primary" style={{ height: '24px', fontSize: '11px', padding: '0 8px' }} onClick={handleSaveIssue}>
                                                        <CheckIcon /> Save
                                                    </button>
                                                    <button className="btn" style={{ height: '24px', fontSize: '11px', padding: '0 8px' }} onClick={() => setIsEditing(false)}>
                                                        Cancel
                                                    </button>
                                                </>
                                            ) : (
                                                <button className="btn-icon" onClick={() => { setIsEditing(true); setEditedDescription(selectedIssue.description || ''); }}>
                                                    <Pencil1Icon />
                                                </button>
                                            )}
                                            <button className="btn-icon" onClick={() => setIsDetailMaximized(!isDetailMaximized)}>
                                                {isDetailMaximized ? <ExitFullScreenIcon /> : <EnterFullScreenIcon />}
                                            </button>
                                            <button className="btn-icon" onClick={() => { setSelectedIssue(null); setIsDetailMaximized(false); setIsEditing(false); }}>
                                                <Cross2Icon />
                                            </button>
                                        </div>
                                    </div>
                                    <h1 className="detail-title">{selectedIssue.title}</h1>
                                    <div className="detail-meta-grid">
                                        <div className="meta-field">
                                            <span className="meta-label">STATUS</span>
                                            <div className="meta-value">
                                                <Select 
                                                    value={selectedIssue.status} 
                                                    onValueChange={async (val) => {
                                                        const [proj, num] = selectedIssue.id.split('/')
                                                        await fetch(`/api/issues/${proj}/${num}`, {
                                                            method: 'PATCH',
                                                            headers: { 'Content-Type': 'application/json' },
                                                            body: JSON.stringify({ status: val })
                                                        })
                                                        fetchData()
                                                        setSelectedIssue(prev => prev ? { ...prev, status: val } : null)
                                                    }}
                                                >
                                                    <SelectTrigger style={{ height: '24px', border: 'none', background: 'transparent', padding: 0 }}>
                                                        <div className="flex items-center gap-2">
                                                            <StatusIcon status={selectedIssue.status} size={12} />
                                                            <span className="capitalize">{selectedIssue.status}</span>
                                                        </div>
                                                    </SelectTrigger>
                                                    <SelectContent>
                                                        {['backlog', 'todo', 'in-progress', 'completed', 'cancelled'].map(s => (
                                                            <SelectItem key={s} value={s} className="capitalize">{s.replace('-', ' ')}</SelectItem>
                                                        ))}
                                                    </SelectContent>
                                                </Select>
                                            </div>
                                        </div>
                                        <div className="meta-field">
                                            <span className="meta-label">PRIORITY</span>
                                            <div className="meta-value">
                                                <Select 
                                                    value={selectedIssue.priority} 
                                                    onValueChange={async (val) => {
                                                        const [proj, num] = selectedIssue.id.split('/')
                                                        await fetch(`/api/issues/${proj}/${num}`, {
                                                            method: 'PATCH',
                                                            headers: { 'Content-Type': 'application/json' },
                                                            body: JSON.stringify({ priority: val })
                                                        })
                                                        fetchData()
                                                        setSelectedIssue(prev => prev ? { ...prev, priority: val } : null)
                                                    }}
                                                >
                                                    <SelectTrigger style={{ height: '24px', border: 'none', background: 'transparent', padding: 0 }}>
                                                        <div className="flex items-center gap-2">
                                                            <PriorityDot priority={selectedIssue.priority} />
                                                            <span className="capitalize">{selectedIssue.priority}</span>
                                                        </div>
                                                    </SelectTrigger>
                                                    <SelectContent>
                                                        {['low', 'medium', 'high', 'urgent'].map(p => (
                                                            <SelectItem key={p} value={p} className="capitalize">{p}</SelectItem>
                                                        ))}
                                                    </SelectContent>
                                                </Select>
                                            </div>
                                        </div>
                                    </div>
                                </header>
                                <div className="flex-1 overflow-y-auto">
                                    <div className="detail-body">
                                        {isEditing ? (
                                            <MarkdownEditor 
                                                content={editedDescription} 
                                                onChange={setEditedDescription}
                                                placeholder="Describe the issue..."
                                            />
                                        ) : (
                                            <div className="markdown-body">
                                                {selectedIssue.description ? (
                                                    <ReactMarkdown remarkPlugins={[remarkGfm]}>{selectedIssue.description}</ReactMarkdown>
                                                ) : (
                                                    <p style={{ fontStyle: 'italic', color: 'var(--fg-subtle)' }}>No description provided.</p>
                                                )}
                                            </div>
                                        )}
                                    </div>
                                </div>
                            </aside>
                        )}
                    </div>
                ) : selectedProjectId ? (
                    <div className="flex-1 overflow-y-auto">
                        <div className="project-detail-view" style={{ padding: '32px' }}>
                            {projects.find(p => p.id === selectedProjectId) ? (() => {
                                const project = projects.find(p => p.id === selectedProjectId)!;
                                const projectIssues = issues.filter(i => i.project === project.id);
                                const projectMilestones = milestones.filter(m => m.project === project.id);
                                const completedIssues = projectIssues.filter(i => i.status === 'completed').length;
                                const progress = projectIssues.length > 0 ? (completedIssues / projectIssues.length) * 100 : 0;

                                return (
                                    <>
                                        <div className="flex justify-between items-start mb-8">
                                            <div>
                                                <button
                                                    className="btn-back"
                                                    onClick={() => setSelectedProjectId(null)}
                                                >
                                                    <DoubleArrowLeftIcon /> Back to All Projects
                                                </button>
                                                <h1 style={{ fontSize: '32px', fontWeight: 800, marginBottom: '8px' }}>{project.name}</h1>
                                                <div className="flex items-center gap-4 text-sm text-subtle">
                                                    <span className="flex items-center gap-2">
                                                        <Select 
                                                            value={project.status} 
                                                            onValueChange={async (val) => {
                                                                await fetch(`/api/projects/${project.id}`, {
                                                                    method: 'PATCH',
                                                                    headers: { 'Content-Type': 'application/json' },
                                                                    body: JSON.stringify({ status: val })
                                                                })
                                                                fetchData()
                                                            }}
                                                        >
                                                            <SelectTrigger style={{ height: '24px', border: 'none', background: 'transparent', padding: 0 }}>
                                                                <div className="flex items-center gap-2">
                                                                    <StatusIcon status={project.status} size={12} />
                                                                    <span className="capitalize">{project.status}</span>
                                                                </div>
                                                            </SelectTrigger>
                                                            <SelectContent>
                                                                {['backlog', 'todo', 'in-progress', 'completed', 'cancelled'].map(s => (
                                                                    <SelectItem key={s} value={s} className="capitalize">{s.replace('-', ' ')}</SelectItem>
                                                                ))}
                                                            </SelectContent>
                                                        </Select>
                                                    </span>
                                                    <span className="flex items-center gap-2">
                                                        <Select 
                                                            value={project.priority} 
                                                            onValueChange={async (val) => {
                                                                await fetch(`/api/projects/${project.id}`, {
                                                                    method: 'PATCH',
                                                                    headers: { 'Content-Type': 'application/json' },
                                                                    body: JSON.stringify({ priority: val })
                                                                })
                                                                fetchData()
                                                            }}
                                                        >
                                                            <SelectTrigger style={{ height: '24px', border: 'none', background: 'transparent', padding: 0 }}>
                                                                <div className="flex items-center gap-2">
                                                                    <PriorityDot priority={project.priority} />
                                                                    <span className="capitalize">{project.priority} priority</span>
                                                                </div>
                                                            </SelectTrigger>
                                                            <SelectContent>
                                                                {['low', 'medium', 'high', 'urgent'].map(p => (
                                                                    <SelectItem key={p} value={p} className="capitalize">{p}</SelectItem>
                                                                ))}
                                                            </SelectContent>
                                                        </Select>
                                                    </span>
                                                    <span>{projectIssues.length} issues • {projectMilestones.length} milestones</span>
                                                </div>
                                            </div>
                                            <div className="flex flex-col items-end gap-2">
                                                <div className="text-sm font-semibold">{Math.round(progress)}% Complete</div>
                                                <div style={{ width: '200px', height: '8px', background: 'var(--bg-element)', borderRadius: '4px', overflow: 'hidden' }}>
                                                    <div style={{ width: `${progress}%`, height: '100%', background: 'var(--status-completed)' }} />
                                                </div>
                                            </div>
                                        </div>

                                        <div style={{ display: 'grid', gridTemplateColumns: '1fr 340px', gap: '48px' }}>
                                            <div className="project-detail-main">
                                                <section className="mb-8">
                                                    <div className="flex justify-between items-center mb-4">
                                                        <h2 className="section-title">Description</h2>
                                                        {isEditing ? (
                                                            <div className="flex gap-2">
                                                                <button className="btn btn-primary" onClick={handleSaveProject}>
                                                                    <CheckIcon /> Save
                                                                </button>
                                                                <button className="btn" onClick={() => setIsEditing(false)}>
                                                                    Cancel
                                                                </button>
                                                            </div>
                                                        ) : (
                                                            <button className="btn-icon" onClick={() => { setIsEditing(true); setEditedDescription(project.description || ''); }}>
                                                                <Pencil1Icon />
                                                                <span style={{ marginLeft: '4px', fontSize: '12px' }}>Edit</span>
                                                            </button>
                                                        )}
                                                    </div>
                                                    {isEditing ? (
                                                        <MarkdownEditor 
                                                            content={editedDescription} 
                                                            onChange={setEditedDescription}
                                                            placeholder="Describe the project..."
                                                        />
                                                    ) : (
                                                        <div className="markdown-body">
                                                            {project.description ? (
                                                                <ReactMarkdown remarkPlugins={[remarkGfm]}>{project.description}</ReactMarkdown>
                                                            ) : (
                                                                <p className="text-subtle italic">No README.md found for this project.</p>
                                                            )}
                                                        </div>
                                                    )}
                                                </section>

                                                <section className="mb-8">
                                                    <div className="flex justify-between items-center mb-4">
                                                        <h2 className="section-title">Milestone Timeline</h2>
                                                    </div>
                                                    <div className="gantt-container">
                                                        <div className="gantt-header">
                                                            <div className="gantt-labels">Milestone</div>
                                                            <div className="gantt-grid">
                                                                {['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'].map(m => (
                                                                    <div key={m} className="gantt-col">{m}</div>
                                                                ))}
                                                            </div>
                                                        </div>
                                                        {projectMilestones.map(m => {
                                                            const date = m.target_date ? new Date(m.target_date) : new Date()
                                                            const month = date.getMonth()
                                                            const left = (month / 12) * 100
                                                            const width = 8
                                                            return (
                                                                <div key={m.id} className="gantt-row">
                                                                    <div className="gantt-milestone-label">
                                                                        <StatusIcon status={m.status} size={10} />
                                                                        <span style={{ marginLeft: '8px' }}>{m.title}</span>
                                                                    </div>
                                                                    <div className="gantt-chart-area">
                                                                        <div
                                                                            className="gantt-bar text-xs"
                                                                            style={{
                                                                                left: `${left}%`,
                                                                                width: `${width}%`,
                                                                                background: m.status === 'completed' ? 'var(--status-completed)' :
                                                                                    m.status === 'in-progress' ? 'var(--status-progress)' : 'var(--accent)'
                                                                            }}
                                                                        >
                                                                            {m.target_date ? date.getDate() : ''}
                                                                        </div>
                                                                    </div>
                                                                </div>
                                                            )
                                                        })}
                                                    </div>
                                                </section>
                                            </div>

                                            <div className="project-detail-sidebar">
                                                <section className="mb-8">
                                                    <div className="flex justify-between items-center mb-4">
                                                        <h3 className="section-title" style={{ margin: 0, border: 'none' }}>Project Issues</h3>
                                                        <span className="text-xs text-subtle">{projectIssues.length} total</span>
                                                    </div>
                                                    <div className="flex flex-col gap-3">
                                                        {projectIssues.slice(0, 15).map(issue => (
                                                            <div
                                                                key={issue.id}
                                                                className="mini-issue-row"
                                                                onClick={() => { setSelectedIssue(issue); setCurrentView('issues'); }}
                                                            >
                                                                <div className="mini-issue-header">
                                                                    <span className="mini-issue-title">{issue.title}</span>
                                                                    <span className="text-xs font-mono text-muted">[{issue.id}]</span>
                                                                </div>
                                                                <div className="mini-issue-footer">
                                                                    <div className="status-pill">
                                                                        <StatusIcon status={issue.status} size={10} />
                                                                        <span className="capitalize">{issue.status}</span>
                                                                    </div>
                                                                    <div className="flex items-center gap-1 text-xs text-subtle">
                                                                        <PriorityDot priority={issue.priority} />
                                                                        <span className="capitalize">{issue.priority}</span>
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        ))}
                                                        {projectIssues.length > 15 && (
                                                            <button
                                                                className="btn-subtle w-full text-xs py-3"
                                                                onClick={() => setCurrentView('issues')}
                                                            >
                                                                View all {projectIssues.length} issues
                                                            </button>
                                                        )}
                                                    </div>
                                                </section>
                                            </div>
                                        </div>
                                    </>
                                );
                            })() : (
                                <div className="empty-state">Project not found</div>
                            )}
                        </div>
                    </div>
                ) : (
                    <div className="flex-1 overflow-y-auto">
                        <div className="view-pane-header" style={{ padding: '16px 24px', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                            <h2 style={{ fontSize: '18px', fontWeight: 600 }}>Projects Overview</h2>
                            <div className="toggle-group">
                                <button
                                    className={`btn-icon ${projectViewMode === 'grid' ? 'active' : ''}`}
                                    onClick={() => setProjectViewMode('grid')}
                                    title="Grid View"
                                >
                                    <ViewGridIcon />
                                </button>
                                <button
                                    className={`btn-icon ${projectViewMode === 'list' ? 'active' : ''}`}
                                    onClick={() => setProjectViewMode('list')}
                                    title="List View"
                                >
                                    <ListBulletIcon />
                                </button>
                            </div>
                        </div>

                        <div className={projectViewMode === 'grid' ? "projects-grid" : "projects-list"}>
                            {groupedProjects.map((group, groupIdx) => (
                                <Fragment key={groupIdx}>
                                    {group.label && (
                                        <div className="project-group-header">
                                            {group.label} ({group.projects.length})
                                        </div>
                                    )}
                                    {group.projects.map(project => {
                                        const projectIssues = issues.filter(i => i.project === project.id)
                                        const projectMilestones = milestones.filter(m => m.project === project.id)
                                        const completedIssues = projectIssues.filter(i => i.status === 'completed').length
                                        const progress = projectIssues.length > 0 ? (completedIssues / projectIssues.length) * 100 : 0

                                        if (projectViewMode === 'list') {
                                            return (
                                                <div key={project.id} className="project-list-row" onClick={() => setSelectedProjectId(project.id)}>
                                                    <div className="project-list-info">
                                                        <span className="project-list-name">{project.name}</span>
                                                        <span className="project-list-stats">{projectIssues.length} issues • {projectMilestones.length} milestones</span>
                                                    </div>
                                                    <div className="project-list-progress">
                                                        <div className="progress-bar-small">
                                                            <div className="progress-fill" style={{ width: `${progress}%` }} />
                                                        </div>
                                                        <span className="progress-pct">{Math.round(progress)}%</span>
                                                    </div>
                                                    <div className="project-list-meta">
                                                        <PriorityDot priority={project.priority} />
                                                        <span className="capitalize">{project.priority}</span>
                                                    </div>
                                                </div>
                                            )
                                        }

                                        return (
                                            <div key={project.id} className="project-card" onClick={() => setSelectedProjectId(project.id)}>
                                                <div className="project-card-header">
                                                    <div>
                                                        <h3 className="project-card-title">{project.name}</h3>
                                                        <div className="project-card-stats">
                                                            <span>{projectIssues.length} issues</span>
                                                            <span>{projectMilestones.length} milestones</span>
                                                        </div>
                                                    </div>
                                                    <PriorityDot priority={project.priority} />
                                                </div>
                                                <div style={{ width: '100%', height: '6px', background: 'var(--bg-element)', borderRadius: '3px', overflow: 'hidden' }}>
                                                    <div style={{ width: `${progress}%`, height: '100%', background: 'var(--accent)' }} />
                                                </div>
                                                <div className="nav-section-title" style={{ padding: 0, marginTop: '8px' }}>Recent Milestones</div>
                                                <div className="flex flex-col gap-2">
                                                    {projectMilestones.slice(0, 3).map(m => (
                                                        <div key={m.id} className="flex items-center justify-between text-xs">
                                                            <span className="flex items-center gap-2">
                                                                <StatusIcon status={m.status} size={10} />
                                                                {m.title}
                                                            </span>
                                                            <span className="text-subtle">{m.target_date || 'No Date'}</span>
                                                        </div>
                                                    ))}
                                                </div>
                                            </div>
                                        )
                                    })}
                                </Fragment>
                            ))}
                        </div>
                        <div className="milestone-timeline" style={{ padding: '0 24px 48px 24px' }}>
                            <h2 style={{ fontSize: '18px', fontWeight: 600, marginBottom: '24px', borderBottom: '1px solid var(--border-muted)', paddingBottom: '12px' }}>Workspace Timeline</h2>
                            <div className="gantt-container">
                                <div className="gantt-header">
                                    <div className="gantt-labels">Milestone</div>
                                    <div className="gantt-grid">
                                        {['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'].map(m => (
                                            <div key={m} className="gantt-col">{m}</div>
                                        ))}
                                    </div>
                                </div>
                                {milestones.filter(m => !selectedProjectId || m.project === selectedProjectId).map(m => {
                                    const date = m.target_date ? new Date(m.target_date) : new Date()
                                    const month = date.getMonth()
                                    const left = (month / 12) * 100
                                    const width = 8

                                    return (
                                        <div key={m.id} className="gantt-row">
                                            <div className="gantt-milestone-label">
                                                <StatusIcon status={m.status} size={10} />
                                                <span style={{ marginLeft: '8px' }}>{m.title}</span>
                                            </div>
                                            <div className="gantt-chart-area">
                                                <div
                                                    className="gantt-bar text-xs"
                                                    style={{
                                                        left: `${left}%`,
                                                        width: `${width}%`,
                                                        opacity: m.status === 'completed' ? 0.6 : 1,
                                                        background: m.status === 'completed' ? 'var(--status-completed)' :
                                                            m.status === 'in-progress' ? 'var(--status-progress)' : 'var(--accent)'
                                                    }}
                                                >
                                                    {m.target_date ? date.getDate() : ''}
                                                </div>
                                            </div>
                                        </div>
                                    )
                                })}
                            </div>
                        </div>
                    </div>
                )}
            </main>

            {/* Modals */}
            {isCreateModalOpen === 'project' && (
                <Modal 
                    title="Create New Project" 
                    onConfirm={handleCreate} 
                    onCancel={() => setIsCreateModalOpen(null)}
                >
                    <div className="form-group">
                        <label className="form-label">Project Name</label>
                        <input 
                            className="form-input" 
                            value={createFormData.name} 
                            onChange={e => setCreateFormData({ ...createFormData, name: e.target.value })}
                            placeholder="e.g. My Great Project"
                        />
                    </div>
                    <div className="form-group">
                        <label className="form-label">Project ID (Optional)</label>
                        <input 
                            className="form-input" 
                            value={createFormData.id} 
                            onChange={e => setCreateFormData({ ...createFormData, id: e.target.value })}
                            placeholder="e.g. MGP"
                        />
                    </div>
                    <div className="form-group">
                        <label className="form-label">Priority</label>
                        <Select 
                            value={createFormData.priority} 
                            onValueChange={val => setCreateFormData({ ...createFormData, priority: val })}
                        >
                            <SelectTrigger className="form-input">
                                <SelectValue />
                            </SelectTrigger>
                            <SelectContent>
                                {['low', 'medium', 'high', 'urgent'].map(p => (
                                    <SelectItem key={p} value={p} className="capitalize">{p}</SelectItem>
                                ))}
                            </SelectContent>
                        </Select>
                    </div>
                </Modal>
            )}

            {isCreateModalOpen === 'milestone' && (
                <Modal 
                    title="Add Milestone" 
                    onConfirm={handleCreate} 
                    onCancel={() => setIsCreateModalOpen(null)}
                >
                    {!selectedProjectId && (
                        <div className="form-group">
                            <label className="form-label">Project</label>
                            <Select 
                                value={createFormData.project} 
                                onValueChange={val => setCreateFormData({ ...createFormData, project: val })}
                            >
                                <SelectTrigger className="form-input">
                                    <SelectValue placeholder="Select Project" />
                                </SelectTrigger>
                                <SelectContent>
                                    {projects.map(p => (
                                        <SelectItem key={p.id} value={p.id}>{p.name}</SelectItem>
                                    ))}
                                </SelectContent>
                            </Select>
                        </div>
                    )}
                    <div className="form-group">
                        <label className="form-label">Milestone Title</label>
                        <input 
                            className="form-input" 
                            value={createFormData.title} 
                            onChange={e => setCreateFormData({ ...createFormData, title: e.target.value })}
                            placeholder="e.g. v1.0"
                        />
                    </div>
                    <div className="form-group">
                        <label className="form-label">Target Date (YYYY-MM-DD)</label>
                        <input 
                            className="form-input" 
                            type="date"
                            value={createFormData.date} 
                            onChange={e => setCreateFormData({ ...createFormData, date: e.target.value })}
                        />
                    </div>
                </Modal>
            )}

            {isCreateModalOpen === 'issue' && (
                <Modal 
                    title="New Issue" 
                    onConfirm={handleCreate} 
                    onCancel={() => setIsCreateModalOpen(null)}
                >
                    <div className="form-group">
                        <label className="form-label">Project</label>
                        <Select 
                            value={createFormData.project} 
                            onValueChange={val => setCreateFormData({ ...createFormData, project: val })}
                        >
                            <SelectTrigger className="form-input" disabled={!!selectedProjectId}>
                                <SelectValue placeholder="Select Project" />
                            </SelectTrigger>
                            <SelectContent>
                                {projects.map(p => (
                                    <SelectItem key={p.id} value={p.id}>{p.name}</SelectItem>
                                ))}
                            </SelectContent>
                        </Select>
                    </div>
                    <div className="form-group">
                        <label className="form-label">Issue Title</label>
                        <input 
                            className="form-input" 
                            value={createFormData.title} 
                            onChange={e => setCreateFormData({ ...createFormData, title: e.target.value })}
                            placeholder="e.g. Fix that bug"
                        />
                    </div>
                    <div className="form-group">
                        <label className="form-label">Priority</label>
                        <Select 
                            value={createFormData.priority} 
                            onValueChange={val => setCreateFormData({ ...createFormData, priority: val })}
                        >
                            <SelectTrigger className="form-input">
                                <SelectValue />
                            </SelectTrigger>
                            <SelectContent>
                                {['low', 'medium', 'high', 'urgent'].map(p => (
                                    <SelectItem key={p} value={p} className="capitalize">{p}</SelectItem>
                                ))}
                            </SelectContent>
                        </Select>
                    </div>
                    <div className="form-group">
                        <label className="form-label">Milestone (Optional)</label>
                        <Select 
                            value={createFormData.milestone} 
                            onValueChange={val => setCreateFormData({ ...createFormData, milestone: val })}
                        >
                            <SelectTrigger className="form-input">
                                <SelectValue placeholder="None" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="_none">None</SelectItem>
                                {milestones.filter(m => m.project === (createFormData.project || selectedProjectId)).map(m => (
                                    <SelectItem key={m.id} value={m.title}>{m.title}</SelectItem>
                                ))}
                            </SelectContent>
                        </Select>
                    </div>
                    <div className="form-group">
                        <label className="form-label">Tags (comma separated)</label>
                        <input 
                            className="form-input" 
                            value={createFormData.tags} 
                            onChange={e => setCreateFormData({ ...createFormData, tags: e.target.value })}
                            placeholder="e.g. bug,ui"
                        />
                    </div>
                </Modal>
            )}
        </div>
    )
}

export default App
