import React from 'react'
import { User } from '../../hooks/useAuth'
import { Item } from './Item'

type Props = {
  friends: User[]
  handleClickFriend: (id: number) => void
}

export const List: React.FC<Props> = ({ friends, handleClickFriend }) => {
  return (
    <div className="mt-2">
      <div className="flex flex-col -mx-4">
        {friends.map((friend) => (
          <Item
            id={friend.id}
            name={friend.name}
            avatarImageUrl={friend.avatarImageUrl}
            onClick={handleClickFriend}
          />
        ))}
      </div>
    </div>
  )
}
