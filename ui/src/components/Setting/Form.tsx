import React from 'react'
import { TextInputWithLabel } from '../Forms/TextInputWithLabel'
import { useForm, SubmitHandler } from 'react-hook-form'
import { Button } from '../Forms/Button'

type Inputs = {
  userId: string
  name: string
}

type Props = {}

export const Form: React.FC<Props> = () => {
  const {
    register,
    handleSubmit,
    formState: { errors }
  } = useForm<Inputs>()
  const onSubmit: SubmitHandler<Inputs> = (data) => {}

  return (
    <form className='py-6'>
      <TextInputWithLabel
        labelText="ユーザーID"
        id="userId"
        {...register('userId')}
      />
      <TextInputWithLabel
        labelText="ユーザー名"
        id="name"
        {...register('name')}
      />
      <Button>Submit</Button>
    </form>
  )
}
