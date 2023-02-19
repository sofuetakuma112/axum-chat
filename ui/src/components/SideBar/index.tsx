import React from 'react'
import { Header } from './Header'
import { Tab } from './Tab'
import { TalkRoomList } from './TalkRoomList'

export const SideBar: React.FC = () => {
    return (
        <div className="flex flex-col w-full h-full pl-4 pr-4 py-4 -mr-4">
            <Header />
            <Tab />
            <TalkRoomList />
        </div>
    )
}
