import classNames from 'classnames'

type Props = {
  id: number
  name: string
  avatarImageUrl: string
  onClick: (id: number) => void
}

export const Item: React.FC<Props> = ({
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
