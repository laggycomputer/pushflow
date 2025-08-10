'use client';
import { Button, Checkbox, Dialog, DialogActions, FormControl, FormControlLabel, FormGroup, FormLabel, TextField } from "@mui/material";
import Card, { CardHeader } from "../Card";
import { ChangeEvent, useEffect, useState } from "react";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { DialogName } from "@/helpers/dialog";
import { closeDialog } from "@/store/slices/dialogSlice";
import { ServiceApiKey } from "@/types";
import { createServiceApiKey } from "@/helpers/service-api-key";

interface CreateApiKeyDialogProps {
  serviceId: string;
  onCreate: (apiKey: ServiceApiKey) => void;
}

export default function CreateApiKeyDialog ({ serviceId, onCreate }: CreateApiKeyDialogProps) {
  const dispatch = useAppDispatch()
  const isOpen = useAppSelector(state => state.dialog.activeDialog === DialogName.NewServiceApiKeyPopup)

  const descriptions = {
    sub: 'Add and Edit Subscribers',
    notify: 'Notify users',
    // group: 'Add and Edit Groups'
  }
  const scopeKeys = Object.keys(descriptions) as (keyof typeof descriptions)[]
  const initialScopeState = Object.fromEntries(scopeKeys.map(k => [k, false]))

  const [submitting, setSubmitting] = useState(false)
  const [keyName, setKeyName] = useState('')
  const [scopes, setScopes] = useState(initialScopeState)
  const [modified, setModified] = useState(false)
  
  const selectedScopes = scopeKeys.filter(key => scopes[key])

  const handleClose = () => dispatch(closeDialog())

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault()
    if (selectedScopes.length === 0) return setModified(true)

    setSubmitting(true)
    const apiKey = await createServiceApiKey(serviceId, keyName, selectedScopes)
    setSubmitting(false)

    if (!apiKey) return console.error('There was an error creating the API Key')

    onCreate(apiKey)
  }

  const handleCheckboxChange = (event: ChangeEvent<HTMLInputElement>) => {
    setScopes({ ...scopes, [event.target.name]: event.target.checked })
    setModified(true)
  }

  useEffect(() => {
    if (!isOpen) return
    setModified(false)
    setScopes(initialScopeState)
  }, [isOpen])

  return <Dialog open={isOpen} onClose={handleClose}>
    <Card>
      <CardHeader text="Create API Key" />
      <form onSubmit={handleSubmit} id="create-api-key-form">
        <TextField
          autoFocus
          required
          margin="dense"
          name="key_name"
          label="API Key Name"
          fullWidth
          onChange={e => setKeyName(e.target.value)}
          disabled={submitting}
        />
        <br />
        <br />

        <FormControl required error={modified && selectedScopes.length === 0}>
          <FormLabel component="legend">Choose at least one API Scope</FormLabel>
          <FormGroup>
            {scopeKeys.map((key) => <FormControlLabel
              label={descriptions[key]}
              key={key}
              control={<Checkbox
                checked={scopes[key]}
                onChange={handleCheckboxChange}
                name={key}
                disabled={submitting}
              />}
            />)}
          </FormGroup>
        </FormControl>
      </form>
      <DialogActions>
        <Button onClick={handleClose} disabled={submitting}>Cancel</Button>
        <Button form="create-api-key-form" type="submit" disabled={submitting} loading={submitting}>
          Create
        </Button>
      </DialogActions>
    </Card>
  </Dialog>
}
