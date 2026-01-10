import * as React from "react"
import { Search } from "lucide-react"

export interface InputProps
    extends React.InputHTMLAttributes<HTMLInputElement> { }

const SearchInput = React.forwardRef<HTMLInputElement, InputProps>(
    ({ className, type, ...props }, ref) => {
        return (
            <div className="ui-search-container">
                <Search size={14} className="ui-search-icon" />
                <input
                    type={type}
                    className="ui-search-input"
                    ref={ref}
                    {...props}
                />
                <div className="ui-search-shortcut">
                    <kbd className="ui-kbd">
                        <span style={{ fontSize: '10px' }}>⌘</span>K
                    </kbd>
                </div>
            </div>
        )
    }
)
SearchInput.displayName = "SearchInput"

export { SearchInput }
