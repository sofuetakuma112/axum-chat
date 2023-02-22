import classNames from 'classnames'
import React, { ComponentPropsWithoutRef } from 'react'
import { Size } from '../../types/style'
import { sizeToClassName } from '../../utils/style'

type Props = {
  size?: Size
} & ComponentPropsWithoutRef<'svg'>

export const SearchIcon: React.FC<Props> = ({ size = "md", ...rest }) => {
  return (
    <svg
    className={classNames('stroke-current', sizeToClassName(size))}
      fill="none"
      stroke="currentColor"
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
      {...rest}
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        strokeWidth={2}
        d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
      />
    </svg>
  )
}
