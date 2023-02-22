import classNames from 'classnames'
import { ComponentPropsWithoutRef } from 'react'
import { Size } from '../../types/style'
import { sizeToClassName } from '../../utils/style'

type Props = {
  size?: Size
} & ComponentPropsWithoutRef<'svg'>

export const LogoutIcon: React.FC<Props> = ({ size = "md", ...rest }) => {
  return (
    <svg
    className={classNames(sizeToClassName(size))}
      fill="none"
      stroke="currentColor"
      strokeWidth="1.5"
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
      aria-hidden="true"
      {...rest}
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15M12 9l-3 3m0 0l3 3m-3-3h12.75"
      ></path>
    </svg>
  )
}
