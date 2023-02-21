import React, { ComponentPropsWithRef } from 'react'
import { Input } from './Input'

type Props = {
  labelText: string
  svgIcon?: React.ReactNode
} & ComponentPropsWithRef<'input'>

export const TextInputWithLabel: React.FC<Props> = React.forwardRef(
  ({ labelText, id, svgIcon, ...rest }, ref) => {
    return (
      <div className="flex flex-col mb-6">
        <label
          htmlFor={id}
          className="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
        >
          {labelText}
        </label>
        <div className="relative">
          {svgIcon && (
            <div className="inline-flex items-center justify-center absolute left-0 top-0 h-full w-10 text-gray-400">
              {svgIcon}
            </div>
          )}
          <Input id={id} {...rest} ref={ref} noSvgIcon={!svgIcon} />
        </div>
      </div>
    )
  }
)
