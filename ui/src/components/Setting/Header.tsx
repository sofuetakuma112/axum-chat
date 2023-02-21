import React from 'react'
import { SearchIcon } from '../Icons/SearchIcon'
import { UserPlusIcon } from '../Icons/UserPlusIcon'

type Props = {}

export const Header: React.FC<Props> = () => {
  return (
    <div className="flex flex-row items-center">
      <div className="flex flex-row items-center">
        <div className="text-xl font-semibold">Settings</div>
      </div>
      {/* <div className="ml-auto flex gap-2">
        <button className="flex items-center justify-center h-7 w-7 bg-gray-200 text-gray-500 rounded-full">
          <UserPlusIcon />
        </button>
        <button className="flex items-center justify-center h-7 w-7 bg-gray-200 text-gray-500 rounded-full">
          <SearchIcon />
        </button>
      </div> */}
    </div>
  )
}
