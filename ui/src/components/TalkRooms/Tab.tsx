import React from 'react'

export const Tab: React.FC = () => {
    return (
        <div className="mt-5">
            <ul className="flex flex-row items-center justify-between">
                <li>
                    <a href="#" className="flex items-center pb-3 text-xs font-semibold relative text-indigo-800">
                        <span>All Conversations</span>
                        <span className="absolute left-0 bottom-0 h-1 w-6 bg-indigo-800 rounded-full" />
                    </a>
                </li>
                <li>
                    <a href="#" className="flex items-center pb-3 text-xs text-gray-700 font-semibold">
                        <span>Archived</span>
                    </a>
                </li>
                <li>
                    <a href="#" className="flex items-center pb-3 text-xs text-gray-700 font-semibold">
                        <span>Starred</span>
                    </a>
                </li>
            </ul>
        </div>
    )
}
