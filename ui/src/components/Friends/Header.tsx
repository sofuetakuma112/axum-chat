import React from 'react'

type Props = {}

export const Header: React.FC<Props> = () => {
  return (
    <div className="flex flex-row items-center">
      <div className="flex flex-row items-center">
        <div className="text-xl font-semibold">Friends</div>
      </div>
      <div className="ml-auto">
        <button className="flex items-center justify-center h-7 w-7 bg-gray-200 text-gray-500 rounded-full">
          <svg
            className="w-4 h-4 stroke-current"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
            />
          </svg>
        </button>
      </div>
    </div>
  )
}
