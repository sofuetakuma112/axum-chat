import React from 'react'
import { SearchIcon } from '../Icons/SearchIcon'
import { UserPlusIcon } from '../Icons/UserPlusIcon'

type Props = {
  handleClickHeaderIcon: (tab: Tab) => void
}

export type Tab = 'List' | 'Search'

export const Header: React.FC<Props> = ({ handleClickHeaderIcon }) => {
  return (
    <div className="flex flex-row items-center">
      <div className="flex flex-row items-center">
        <div className="text-xl font-semibold">Friends</div>
      </div>
      <div className="ml-auto flex gap-2">
        <button
          onClick={() => handleClickHeaderIcon('Search')}
          className="flex items-center justify-center h-7 w-7 bg-gray-200 text-gray-500 rounded-full"
        >
          <UserPlusIcon />
        </button>
        <button className="flex items-center justify-center h-7 w-7 bg-gray-200 text-gray-500 rounded-full">
          <SearchIcon />
        </button>
      </div>
    </div>
  )
}
