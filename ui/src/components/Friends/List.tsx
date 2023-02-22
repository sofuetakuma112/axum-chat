import React from 'react'
import { User } from '../../hooks/useAuth'
import { Item } from './Item'

type Props = {
  users: User[]
  handleClickFriend: (id: number) => void
}

export const List: React.FC<Props> = ({ users, handleClickFriend }) => {
  return (
    <div className="mt-2">
      <div className="flex flex-col -mx-4">
        {users.map((user) => (
          <Item
            key={user.id}
            id={user.id}
            name={user.name}
            avatarImageUrl={user.avatarImageUrl}
            onClick={handleClickFriend}
          />
        ))}
      </div>
    </div>
  )
}
