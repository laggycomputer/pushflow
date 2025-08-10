'use client';
import { Button, ButtonGroup, Dialog, DialogActions, DialogContent, DialogContentText, DialogTitle, TextField } from "@mui/material";
import Card, { CardHeader } from "./components/Card";
import { useState } from "react";
import { createService } from "@/helpers/service";
import { useRouter } from "next/navigation";

export default function NewServicePopup () {
  const [open, setOpen] = useState(true)
  const [submitting, setSubmitting] = useState(false)
  const [newServiceName, setNewServiceName] = useState('')
  const router = useRouter()

  const handleClose = () => {
    setOpen(false)
  }

  const handleSubmit = async (event: React.FormEvent) => {
    setSubmitting(true)
    event.preventDefault()
    const service = await createService(newServiceName)
    if (service) return router.push('/projects/' + service.service_id)
    console.error('There was an error creating the service')
    setSubmitting(false)
  }

  return <Dialog open={open} onClose={handleClose}>
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
