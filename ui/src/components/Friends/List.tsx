import classNames from 'classnames'
import React from 'react'
import { User } from '../../hooks/useAuth'

type ItemProps = {
  id: number
  name: string
  avatarImageUrl: string
  onClick: (id: number) => void
}

const Item: React.FC<ItemProps> = ({
  id,
  name,
  avatarImageUrl,
  onClick
}) => (
  <div
    className={classNames('flex flex-row items-center p-4')}
    onClick={() => onClick(id)}
  >
    <img
      className="w-10 h-10 rounded-full"
      src={avatarImageUrl}
      alt="Rounded avatar"
    />

    <div className="flex flex-col flex-grow ml-3">
      <div className="flex items-center">
        <div className="text-sm font-medium">{name}</div>
      </div>
    </div>
  </div>
)

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
