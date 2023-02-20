import React from 'react'
import { Room } from '../../types/type'
import { Header } from './Header'
import { TalkRoomList } from './TalkRoomList'

type Props = {
  totalUnreadMessageCount: number
  rooms: Room[]
  currentRoomId: number
}

export const TalkRooms: React.FC<Props> = ({
  totalUnreadMessageCount,
  rooms,
  currentRoomId
}) => {
  return (
    <div className="flex flex-col w-full h-full px-4 pt-4">
      <Header totalUnreadMessageCount={totalUnreadMessageCount} />
      <TalkRoomList
        rooms={rooms}
        currentRoomId={currentRoomId}
        handleClickRoom={function (id: number): void {
          throw new Error('Function not implemented.')
        }}
      />
    </div>
  )
}
