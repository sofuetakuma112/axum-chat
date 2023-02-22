import { Dialog, Transition } from '@headlessui/react'
import { Fragment } from 'react'

type Props = {
  isOpen: boolean
  handleClose: () => void
  children: React.ReactNode
}

export const Modal: React.FC<Props> = ({ isOpen, handleClose, children }) => {
  return (
    <Dialog
      open={isOpen}
      as="div"
      className="relative z-10"
      onClose={handleClose}
    >
      <div className="fixed inset-0 overflow-y-auto">
        <div className="flex min-h-full items-center justify-center p-4 text-center">
          <Dialog.Panel className="w-full max-w-md transform overflow-hidden rounded-2xl bg-white p-6 text-left align-middle shadow-xl transition-all">
            {children}
          </Dialog.Panel>
        </div>
      </div>
    </Dialog>
  )
}
