'use client';
import { Button, Dialog, DialogActions, TextField } from "@mui/material";
import Card, { CardHeader } from "../Card";
import { useState } from "react";
import { createService } from "@/helpers/service";
import { useRouter } from "next/navigation";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { DialogName } from "@/helpers/dialog";
import { closeDialog } from "@/store/slices/dialogSlice";

export default function NewServiceDialog () {
  const dispatch = useAppDispatch()
  const isOpen = useAppSelector(state => state.dialog.activeDialog === DialogName.NewServicePopup)
  
  const [submitting, setSubmitting] = useState(false)
  const [newServiceName, setNewServiceName] = useState('')
  const router = useRouter()
  
  const handleClose = () => dispatch(closeDialog())

  const handleSubmit = async (event: React.FormEvent) => {
    setSubmitting(true)
    event.preventDefault()
    const service = await createService(newServiceName)
    if (service) return router.push('/projects/' + service.service_id)
    console.error('There was an error creating the service')
    setSubmitting(false)
    handleClose()
  }

  return <Dialog open={isOpen} onClose={handleClose}>
    <Card>
      <CardHeader text="Create New Service" />
      <form onSubmit={handleSubmit} id="new-service-form">
        <TextField
          autoFocus
          required
          margin="dense"
          name="service_name"
          label="Service Name"
          fullWidth
          onChange={e => setNewServiceName(e.target.value)}
          disabled={submitting}
        />
      </form>
      <DialogActions>
        <Button onClick={handleClose} disabled={submitting}>Cancel</Button>
        <Button form="new-service-form" type="submit" disabled={submitting} loading={submitting}>
          Create
        </Button>
      </DialogActions>
    </Card>
  </Dialog>
}
