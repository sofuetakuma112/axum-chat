import React from 'react'
import { ChatBubbleBottomCenterTextIcon } from './Icons/ChatBubbleBottomCenterTextIcon'
import { ChatBubbleOvalLeftEllipsisIcon } from './Icons/ChatBubbleOvalLeftEllipsisIcon'
import { LogoutIcon } from './Icons/LogoutIcon'
import { SettingIcon } from './Icons/SettingIcon'
import { UsersIcon } from './Icons/UsersIcon'

const ListItem: React.FC<{ children: React.ReactNode }> = ({ children }) => (
  <li>
    <span className="flex items-center">
      <span className="flex items-center justify-center text-indigo-100 hover:bg-indigo-700 h-12 w-12 rounded-2xl">
        {children}
      </span>
    </span>
  </li>
)

export const MenuBar: React.FC = () => {
  return (
    <div className="flex flex-col items-center py-4 flex-shrink-0 w-20 bg-indigo-800 rounded-3xl">
      <span className="flex items-center justify-center h-12 w-12 bg-indigo-100 text-indigo-800 rounded-full">
        <ChatBubbleBottomCenterTextIcon />
      </span>
      <ul className="flex flex-col space-y-2 mt-12">
        <ListItem>
          <UsersIcon />
        </ListItem>
        <ListItem>
          <ChatBubbleOvalLeftEllipsisIcon />
        </ListItem>
        <ListItem>
          <SettingIcon />
        </ListItem>
      </ul>
      <button className="mt-auto flex items-center justify-center hover:text-indigo-100 text-indigo-500 h-10 w-10">
        <LogoutIcon />
      </button>
    </div>
  )
}
