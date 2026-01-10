import { useState, useEffect, useMemo } from 'react'
import {
    Layout,
    Inbox,
    Calendar,
    ChevronRight,
    Filter,
    RefreshCw,
    AlertCircle,
    X,
    Maximize2,
    Minimize2,
    Moon,
    Sun,
    CheckCircle2,
    Circle,
    Clock,
    SortAsc,
    Layers,
} from 'lucide-react'
import { ScrollArea } from './components/ui/ScrollArea'
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from './components/ui/Select'
import { SearchInput } from './components/ui/SearchInput'

// Types
interface Project {
    id: string
    name: string
    status: string
    priority: string
}

interface Milestone {
    id: string
    title: string
    status: string
    project: string
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
        case 'completed': return <CheckCircle2 size={size} className="status-completed" />
        case 'in-progress': return <Clock size={size} className="status-progress" />
        case 'cancelled': return <X size={size} className="status-cancelled" />
        default: return <Circle size={size} className="status-todo" />
    }
}

const PriorityDot = ({ priority }: { priority: string }) => {
    return <span className={`priority-badge-dot priority-${priority}`} />
}

function App() {
    const [theme, setTheme] = useState<'dark' | 'light'>('dark')
    const [projects, setProjects] = useState<Project[]>([])
    const [milestones, setMilestones] = useState<Milestone[]>([])
    const [issues, setIssues] = useState<Issue[]>([])
    const [loading, setLoading] = useState(true)

    const [selectedProjectId, setSelectedProjectId] = useState<string | null>(null)
    const [selectedMilestoneId, setSelectedMilestoneId] = useState<string | null>(null)
    const [selectedIssue, setSelectedIssue] = useState<Issue | null>(null)

    const [statusFilter, setStatusFilter] = useState('all')
    const [priorityFilter, setPriorityFilter] = useState('all')
    const [searchTerm, setSearchTerm] = useState('')
    const [isDetailMaximized, setIsDetailMaximized] = useState(false)

    const [sortBy, setSortBy] = useState('number')
    const [groupBy, setGroupBy] = useState('none')

    useEffect(() => {
        document.documentElement.setAttribute('data-theme', theme)
    }, [theme])

    useEffect(() => {
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
        fetchData()
    }, [])

    const processedIssues = useMemo(() => {
        // 1. Filter
        let result = issues.filter(issue => {
            if (selectedProjectId && issue.project !== selectedProjectId) return false
            if (selectedMilestoneId && issue.milestone !== selectedMilestoneId) return false
            if (statusFilter !== 'all' && issue.status !== statusFilter) return false
            if (priorityFilter !== 'all' && issue.priority !== priorityFilter) return false
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

    return (
        <div className="app-container">
            {/* Sidebar */}
            <aside className="sidebar">
                <div className="sidebar-header">
                    <div className="logo">PILLAR</div>
                </div>
                <ScrollArea className="sidebar-content">
                    <div className="nav-section">
                        <div className="nav-section-title">Workspace</div>
                        <div
                            className={`nav-item ${selectedProjectId === null ? 'active' : ''}`}
                            onClick={() => { setSelectedProjectId(null); setSelectedMilestoneId(null); }}
                        >
                            <Inbox size={18} />
                            <span>All Issues</span>
                        </div>
                    </div>

                    <div className="nav-section">
                        <div className="nav-section-title">Projects</div>
                        {projects.map(project => (
                            <div
                                key={project.id}
                                className={`nav-item ${selectedProjectId === project.id ? 'active' : ''}`}
                                onClick={() => { setSelectedProjectId(project.id); setSelectedMilestoneId(null); }}
                            >
                                <Layout size={18} />
                                <span>{project.name}</span>
                            </div>
                        ))}
                    </div>

                    {selectedProjectId && (
                        <div className="nav-section">
                            <div className="nav-section-title">Milestones</div>
                            {milestones.filter(m => m.project === selectedProjectId).map(milestone => (
                                <div
                                    key={milestone.id}
                                    className={`nav-item ${selectedMilestoneId === milestone.id ? 'active' : ''}`}
                                    onClick={() => setSelectedMilestoneId(milestone.id)}
                                >
                                    <Calendar size={18} />
                                    <span>{milestone.title}</span>
                                </div>
                            ))}
                        </div>
                    )}
                </ScrollArea>

                <div className="sidebar-footer" style={{ padding: '16px', borderTop: '1px solid var(--border-muted)' }}>
                    <div className="nav-item" onClick={toggleTheme}>
                        {theme === 'dark' ? <Sun size={18} /> : <Moon size={18} />}
                        <span>{theme === 'dark' ? 'Light Mode' : 'Dark Mode'}</span>
                    </div>
                </div>
            </aside>

            {/* Main Content */}
            <main className="main-content">
                <header className="header">
                    <div className="header-left">
                        <div className="breadcrumb">
                            <span className="fg-subtle">Workspace</span>
                            {selectedProjectId && (
                                <>
                                    <span className="sep"><ChevronRight size={14} /></span>
                                    <span>{projects.find(p => p.id === selectedProjectId)?.name}</span>
                                </>
                            )}
                            {selectedMilestoneId && (
                                <>
                                    <span className="sep"><ChevronRight size={14} /></span>
                                    <span className="fg-subtle">{milestones.find(m => m.id === selectedMilestoneId)?.title}</span>
                                </>
                            )}
                        </div>
                    </div>
                </header>

                {/* Filter Bar */}
                <div className="filter-bar">
                    <div className="filter-group">
                        <Filter size={12} className="text-subtle" />

                        <Select value={statusFilter} onValueChange={setStatusFilter}>
                            <SelectTrigger>
                                <SelectValue placeholder="Status" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="all">Any Status</SelectItem>
                                <SelectItem value="todo">Todo</SelectItem>
                                <SelectItem value="in-progress">In Progress</SelectItem>
                                <SelectItem value="completed">Completed</SelectItem>
                                <SelectItem value="backlog">Backlog</SelectItem>
                                <SelectItem value="cancelled">Cancelled</SelectItem>
                            </SelectContent>
                        </Select>

                        <div className="divider-v" />

                        <Select value={priorityFilter} onValueChange={setPriorityFilter}>
                            <SelectTrigger>
                                <SelectValue placeholder="Priority" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="all">Any Priority</SelectItem>
                                <SelectItem value="high">High</SelectItem>
                                <SelectItem value="medium">Medium</SelectItem>
                                <SelectItem value="low">Low</SelectItem>
                                <SelectItem value="urgent">Urgent</SelectItem>
                            </SelectContent>
                        </Select>

                        <div className="divider-v" />

                        <SortAsc size={12} className="text-subtle" />
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

                        <Layers size={12} className="text-subtle" />
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
                    </div>
                    <div className="filter-results">
                        {processedIssues.length} issues
                    </div>
                </div>

                <div className="view-pane-container">
                    {/* Issue List View */}
                    <ScrollArea className="flex-1">
                        <div className="issue-table">
                            {loading ? (
                                <div className="loading-overlay">
                                    <RefreshCw size={24} className="animate-spin" />
                                    <span>Loading...</span>
                                </div>
                            ) : processedIssues.length === 0 ? (
                                <div className="empty-state">
                                    <AlertCircle size={32} style={{ marginBottom: '1rem', opacity: 0.5 }} />
                                    <h3>No items match your filters</h3>
                                    <p>Try clearing your status or priority filters.</p>
                                </div>
                            ) : (
                                <div className="issue-list-content">
                                    {groupedIssues.map((group, groupIdx) => (
                                        <div key={groupIdx} className="issue-group">
                                            {group.label && (
                                                <div className="issue-group-header">
                                                    {group.label}
                                                    <span className="group-count">{group.issues.length}</span>
                                                </div>
                                            )}
                                            {group.issues.map((issue: Issue) => (
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
                                    ))}
                                </div>
                            )}
                        </div>
                    </ScrollArea>

                    {/* Issue Detail Panel */}
                    {selectedIssue && (
                        <aside className={`detail-panel ${isDetailMaximized ? 'maximized' : ''}`}>
                            <header className="detail-header">
                                <div className="detail-actions">
                                    <span className="issue-number">#{selectedIssue.number}</span>
                                    <div className="flex items-center gap-2">
                                        <button className="btn-icon" onClick={() => setIsDetailMaximized(!isDetailMaximized)}>
                                            {isDetailMaximized ? <Minimize2 size={16} /> : <Maximize2 size={16} />}
                                        </button>
                                        <button className="btn-icon" onClick={() => { setSelectedIssue(null); setIsDetailMaximized(false); }}>
                                            <X size={18} />
                                        </button>
                                    </div>
                                </div>
                                <h1 className="detail-title">{selectedIssue.title}</h1>
                                <div className="detail-meta-grid">
                                    <div className="meta-field">
                                        <span className="meta-label">STATUS</span>
                                        <div className="meta-value">
                                            <StatusIcon status={selectedIssue.status} size={12} />
                                            <span className="capitalize">{selectedIssue.status}</span>
                                        </div>
                                    </div>
                                    <div className="meta-field">
                                        <span className="meta-label">PRIORITY</span>
                                        <div className="meta-value">
                                            <PriorityDot priority={selectedIssue.priority} />
                                            <span className="capitalize">{selectedIssue.priority}</span>
                                        </div>
                                    </div>
                                </div>
                            </header>
                            <ScrollArea className="flex-1">
                                <div className="detail-body">
                                    <div className="markdown-body">
                                        {selectedIssue.description ? (
                                            selectedIssue.description.split('\n').map((line: string, i: number) => {
                                                if (line.startsWith('# ')) return <h1 key={i}>{line.substring(2)}</h1>
                                                if (line.startsWith('## ')) return <h2 key={i}>{line.substring(3)}</h2>
                                                if (line.startsWith('### ')) return <h3 key={i}>{line.substring(4)}</h3>
                                                if (line.startsWith('- ')) return <li key={i}>{line.substring(2)}</li>
                                                if (line.trim() === '') return <br key={i} />
                                                return <p key={i}>{line}</p>
                                            })
                                        ) : (
                                            <p style={{ fontStyle: 'italic', color: 'var(--fg-subtle)' }}>No description provided.</p>
                                        )}
                                    </div>
                                </div>
                            </ScrollArea>
                        </aside>
                    )}
                </div>
            </main>
        </div>
    )
}

export default App
