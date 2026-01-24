import { useEditor, EditorContent } from '@tiptap/react'
import StarterKit from '@tiptap/starter-kit'
import Placeholder from '@tiptap/extension-placeholder'
import { Markdown } from 'tiptap-markdown'
import {
    Bold, Italic, List, ListOrdered, Quote, Heading1, Heading2, Code
} from 'lucide-react'

interface MarkdownEditorProps {
    content: string
    onChange: (markdown: string) => void
    placeholder?: string
}

export const MarkdownEditor = ({ content, onChange, placeholder = 'Write something...' }: MarkdownEditorProps) => {
    const editor = useEditor({
        extensions: [
            StarterKit,
            Markdown,
            Placeholder.configure({
                placeholder,
            }),
        ],
        content,
        onUpdate: ({ editor }) => {
            onChange((editor.storage as any).markdown.getMarkdown())
        },
    })

    if (!editor) {
        return null
    }

    const ToolbarButton = ({ onClick, isActive, children, title }: any) => (
        <button
            onClick={(e) => { e.preventDefault(); onClick(); }}
            className={`btn-icon ${isActive ? 'active' : ''}`}
            title={title}
            style={{ 
                background: isActive ? 'var(--bg-element)' : 'transparent',
                color: isActive ? 'var(--accent)' : 'var(--fg-subtle)',
                width: '32px',
                height: '32px'
            }}
        >
            {children}
        </button>
    )

    return (
        <div className="editor-container">
            <div className="editor-toolbar">
                <ToolbarButton
                    onClick={() => editor.chain().focus().toggleBold().run()}
                    isActive={editor.isActive('bold')}
                    title="Bold"
                >
                    <Bold size={16} />
                </ToolbarButton>
                <ToolbarButton
                    onClick={() => editor.chain().focus().toggleItalic().run()}
                    isActive={editor.isActive('italic')}
                    title="Italic"
                >
                    <Italic size={16} />
                </ToolbarButton>
                <ToolbarButton
                    onClick={() => editor.chain().focus().toggleHeading({ level: 1 }).run()}
                    isActive={editor.isActive('heading', { level: 1 })}
                    title="Heading 1"
                >
                    <Heading1 size={16} />
                </ToolbarButton>
                <ToolbarButton
                    onClick={() => editor.chain().focus().toggleHeading({ level: 2 }).run()}
                    isActive={editor.isActive('heading', { level: 2 })}
                    title="Heading 2"
                >
                    <Heading2 size={16} />
                </ToolbarButton>
                <ToolbarButton
                    onClick={() => editor.chain().focus().toggleBulletList().run()}
                    isActive={editor.isActive('bulletList')}
                    title="Bullet List"
                >
                    <List size={16} />
                </ToolbarButton>
                <ToolbarButton
                    onClick={() => editor.chain().focus().toggleOrderedList().run()}
                    isActive={editor.isActive('orderedList')}
                    title="Ordered List"
                >
                    <ListOrdered size={16} />
                </ToolbarButton>
                <ToolbarButton
                    onClick={() => editor.chain().focus().toggleBlockquote().run()}
                    isActive={editor.isActive('blockquote')}
                    title="Blockquote"
                >
                    <Quote size={16} />
                </ToolbarButton>
                <ToolbarButton
                    onClick={() => editor.chain().focus().toggleCodeBlock().run()}
                    isActive={editor.isActive('codeBlock')}
                    title="Code Block"
                >
                    <Code size={16} />
                </ToolbarButton>
            </div>
            <EditorContent editor={editor} className="tiptap-content" />
        </div>
    )
}
