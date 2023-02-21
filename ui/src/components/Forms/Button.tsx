import React, { ComponentPropsWithoutRef } from 'react'

type Props = {
  svgIcon?: React.ReactNode
  children: React.ReactNode
} & ComponentPropsWithoutRef<'button'>

export const Button: React.FC<Props> = ({ svgIcon, children, ...rest }) => {
  return (
    <button
      className="w-full flex items-center justify-center transition duration-150 ease-in text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
      {...rest}
    >
      <span>{children}</span>
      {svgIcon && <span className="pl-2">{svgIcon}</span>}
    </button>
  )
}
