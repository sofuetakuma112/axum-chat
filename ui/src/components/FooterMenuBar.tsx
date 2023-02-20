import React, { useState } from 'react'
import { ChatBubbleOvalLeftEllipsisIcon } from './Icons/ChatBubbleOvalLeftEllipsisIcon'
import { SettingIcon } from './Icons/SettingIcon'
import { UsersIcon } from './Icons/UsersIcon'
import classNames from 'classnames'

export type Menu = 'FriendList' | 'TalkRoomList' | 'Settings'

const ListItem: React.FC<{
  isCurrent: boolean
  children: React.ReactNode
  onClick: () => void
}> = ({ isCurrent, children, onClick }) => (
  <li className="mr-2 flex-1" onClick={onClick}>
    <span
      className={classNames(
        'inline-flex p-4 border-b-2 border-transparent rounded-t-lg group',
        {
          'text-blue-600 border-blue-600 active dark:text-blue-500 dark:border-blue-500':
            isCurrent
        }
        // {
        //   'hover:text-gray-600 hover:border-gray-300 dark:hover:text-gray-300':
        //     !isCurrent
        // }
      )}
    >
      {children}
    </span>
  </li>
)

type Props = {
  currentMenu: Menu
  handleClickMenu: (menu: Menu) => void
}

export const FooterMenuBar: React.FC<Props> = ({
  currentMenu,
  handleClickMenu
}) => {
  return (
    <div className="border-b border-gray-200 dark:border-gray-700 mt-auto">
      <ul className="flex flex-wrap -mb-px text-sm font-medium text-center text-gray-500 dark:text-gray-400">
        <ListItem
          isCurrent={currentMenu === 'FriendList'}
          onClick={() => handleClickMenu('FriendList')}
        >
          <UsersIcon />
        </ListItem>
        <ListItem
          isCurrent={currentMenu === 'TalkRoomList'}
          onClick={() => handleClickMenu('TalkRoomList')}
        >
          <ChatBubbleOvalLeftEllipsisIcon />
        </ListItem>
        <ListItem
          isCurrent={currentMenu === 'Settings'}
          onClick={() => handleClickMenu('Settings')}
        >
          <SettingIcon />
        </ListItem>
      </ul>
    </div>
  )
}
