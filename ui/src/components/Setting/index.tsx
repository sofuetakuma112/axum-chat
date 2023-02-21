import React from 'react'
import { Header } from './Header'
import { Form } from './Form'

type Props = {}

export const Setting: React.FC<Props> = () => {
  return (
    <div className="flex flex-col w-full h-full px-4 pt-4">
      <Header />
      <Form />
    </div>
  )
}
