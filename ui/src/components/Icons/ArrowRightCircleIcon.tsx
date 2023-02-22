import React, { ComponentPropsWithoutRef } from 'react'

type Props = ComponentPropsWithoutRef<'svg'>

export const ArrowRightCircleIcon: React.FC<Props> = (props) => {
  return (
    <svg
      className="w-6 h-6"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.5"
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
      aria-hidden="true"
      {...props}
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M12.75 15l3-3m0 0l-3-3m3 3h-7.5M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
      ></path>
    </svg>
  )
}
