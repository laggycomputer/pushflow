'use client';
import { Button, Checkbox, Dialog, DialogActions, FormControl, FormControlLabel, FormGroup, FormLabel, TextField } from "@mui/material";
import Card, { CardHeader } from "../Card";
import { ChangeEvent, useEffect, useState } from "react";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { DialogName } from "@/helpers/dialog";
import { closeDialog } from "@/store/slices/dialogSlice";
import { ServiceApiKey } from "@/types";
import { createServiceApiKey, updateApiKey } from "@/helpers/service-api-key";
import { editApiKey } from "@/store/slices/serviceSlice";

interface CreateApiKeyDialogProps {
  serviceId: string;
  onCreate: (apiKey: ServiceApiKey) => void;
}

export default function CreateApiKeyDialog ({ serviceId, onCreate }: CreateApiKeyDialogProps) {
  const dispatch = useAppDispatch()
  const isOpen = useAppSelector(state => state.dialog.activeDialog === DialogName.NewServiceApiKeyPopup)
  const apiKeyId = useAppSelector(state => state.dialog.key)
  const editingApiKey = useAppSelector(state => state.service.apiKeys.find(k => k.key_id === apiKeyId))
  

  const descriptions = {
    sub: 'Add and Edit Subscribers',
    notify: 'Notify users',
    // group: 'Add and Edit Groups'
  }
  const scopeKeys = Object.keys(descriptions) as (keyof typeof descriptions)[]
  const initialScopeState = Object.fromEntries(
    /** @todo refactor */
    scopeKeys.map(k => [k, !!editingApiKey && !!editingApiKey.scopes.find(s => s.scope === k)])
  )

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

    if (editingApiKey) {
      await updateApiKey(serviceId, apiKeyId, keyName, selectedScopes)
      setSubmitting(false)
      
      handleClose()
      const scopes = selectedScopes.map(s => ({ scope: s }))
      dispatch(editApiKey({ ...editingApiKey, name: keyName, scopes }))
      return
    }

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

  if (apiKeyId && !editingApiKey) return null

  return <Dialog open={isOpen} onClose={handleClose}>
    <Card>
      <CardHeader text={editingApiKey ? 'Editing API Key' : 'Create API Key'} />
      <form onSubmit={handleSubmit} id="create-api-key-form">
        <TextField
          autoFocus
          required
          margin="dense"
          name="key_name"
          label="API Key Name"
          fullWidth
          defaultValue={editingApiKey?.name ?? ''}
          onChange={e => setKeyName(e.target.value)}
          disabled={submitting}
        />
        <br />
        <br />

        {!editingApiKey && <FormControl required error={modified && selectedScopes.length === 0}>
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
        </FormControl>}
      </form>
      <DialogActions>
        <Button onClick={handleClose} disabled={submitting}>Cancel</Button>
        <Button form="create-api-key-form" type="submit" disabled={submitting} loading={submitting}>
          {editingApiKey ? 'Save' : 'Create'}
        </Button>
      </DialogActions>
    </Card>
  </Dialog>
}
