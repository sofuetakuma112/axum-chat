import classNames from 'classnames'
import React from 'react'
import { Room } from '../../types/type'
import { diffTime } from '../../utils/date'

type RoomItemProps = {
  isCurrentRoom: boolean
  onClick: (id: number) => void
} & Room

const RoomItem: React.FC<RoomItemProps> = ({
  id,
  name,
  lastMessage,
  unreadMessageCount,
  lastMessageTimestamp,
  isCurrentRoom,
  onClick
}) => (
  <div
    className={classNames('flex flex-row items-center p-4', {
      'bg-gradient-to-r from-red-100 to-transparent border-l-2 border-red-500':
        isCurrentRoom
    })}
    onClick={() => onClick(id)}
  >
    <div className="absolute text-xs text-gray-500 right-0 top-0 mr-4 mt-3">
      {diffTime(new Date(), lastMessageTimestamp)}
    </div>
    <div className="flex items-center justify-center h-10 w-10 rounded-full bg-pink-500 text-pink-300 font-bold flex-shrink-0">
      T
    </div>
    <div className="flex flex-col flex-grow ml-3">
      <div className="flex items-center">
        <div className="text-sm font-medium">{name}</div>
        {/* <div className="h-2 w-2 rounded-full bg-green-500 ml-2" /> */}
        <div className="h-2 w-2 rounded-full bg-green-500 ml-2" />
      </div>
      <div className="text-xs truncate w-40">{lastMessage}</div>
    </div>
    {unreadMessageCount !== 0 && (
      <div className="flex-shrink-0 ml-2 self-end mb-1">
        <span className="flex items-center justify-center h-5 w-5 bg-red-500 text-white text-xs rounded-full">
          {unreadMessageCount}
        </span>
      </div>
    )}
  </div>
)

type Props = {
  rooms: Room[]
  currentRoomId: number
  handleClickRoom: (id: number) => void
}

export const TalkRoomList: React.FC<Props> = ({
  rooms,
  currentRoomId,
  handleClickRoom
}) => {
  return (
    <div className="mt-2">
      <div className="flex flex-col -mx-4">
        {rooms.map((room) => (
          <RoomItem
            id={room.id}
            name={room.name}
            lastMessage={room.lastMessage}
            unreadMessageCount={room.unreadMessageCount}
            lastMessageTimestamp={room.lastMessageTimestamp}
            isCurrentRoom={room.id === currentRoomId}
            onClick={handleClickRoom}
          />
        ))}
      </div>
    </div>
  )
}
