import classNames from 'classnames'
import React, { ComponentPropsWithRef } from 'react'

type Props = {
  noSvgIcon?: boolean
} & ComponentPropsWithRef<'input'>

export const Input: React.FC<Props> = React.forwardRef(
  ({ noSvgIcon = true, ...rest }, ref) => {
    return (
      <input
        className={classNames(
          'bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500',
          { 'pl-10 pr-4': !noSvgIcon }
        )}
        {...rest}
        ref={ref}
      />
    )
  }
)
