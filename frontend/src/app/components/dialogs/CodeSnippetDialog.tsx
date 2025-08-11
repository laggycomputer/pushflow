import './CodeSnippetDialog.scss'
import { Button, Dialog, DialogActions, DialogContentText, Divider, FormControl, Icon, InputLabel, MenuItem, Select } from "@mui/material";
import Card, { CardHeader } from "../Card";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { closeDialog } from "@/store/slices/dialogSlice";

import { DialogName } from '@/helpers/dialog';
import { getServiceWorkerBaseCode, getServiceWorkerRegistrationCode, getSubscribeCode } from "@/helpers/snippet";
import Copyable from "../Copyable";
import config from '@/env';
import React, { useState } from 'react';
import { registerWorker, subscribeToNotifications } from '@/helpers/webpush-test';

import CheckIcon from '@mui/icons-material/Check';
import { ServiceGroup } from '@/types';

export default function ApiKeyCodeSnippetDialog () {
  const dispatch = useAppDispatch()
  const isOpen = useAppSelector(state => state.dialog.activeDialog === DialogName.ShowServiceCodeSnippetPopup)
  const serviceId = useAppSelector(state => state.service.currentServiceId)
  const serviceName = useAppSelector(state => state.service.name)
  const vapidPublicKey = useAppSelector(state => state.service.vapidPublic)
  const groups = useAppSelector(state => state.service.groups)

  const [selectedGroups, setSelectedGroups] = useState<ServiceGroup[]>([])
  const [notifyGroup, setNotifyGroup] = useState<ServiceGroup | ''>('')
  
  if (!serviceId) return null

  const handleClose = () => dispatch(closeDialog())

  const handleTestSubscribe = async () => {
    await registerWorker()
    const groupIds = selectedGroups.map(s => s.group_id)
    const apiKey = process.env.NEXT_PUBLIC_TEST_API_KEY!
    subscribeToNotifications(serviceId, groupIds, vapidPublicKey, apiKey)
  }

  const handleGroupSelectionChange = (event: any) => {
    setSelectedGroups(event.target.value)
  }
  
  return <Dialog open={isOpen} onClose={handleClose} id="code-snippet-popup">
    <Card className="list-container">
      <CardHeader text="Integrate API Key" />
      <h3>Register a Service Worker</h3>
      <DialogContentText>
        Create a <code>/service-worker.js</code> file.
      </DialogContentText>
      <Copyable multiline>
        {getServiceWorkerBaseCode(serviceName)}
      </Copyable>
      <br />
      <DialogContentText>
        Call this function in your frontend code to register the service worker.
      </DialogContentText>
      <Copyable multiline>
        {getServiceWorkerRegistrationCode()}
      </Copyable><h3>Subscribing to Notifications</h3>
      <DialogContentText>
        Choose groups you would like to subscribe a user to.
      </DialogContentText>
      <FormControl fullWidth size="small" margin="dense">
        <InputLabel id="code-snippet-groups-select">Groups</InputLabel>
        <Select
          labelId="code-snippet-groups-select"
          value={selectedGroups}
          label="Groups"
          renderValue={() => selectedGroups.map(g => g.name).join(', ')}
          multiple
          onChange={handleGroupSelectionChange}
        >
          {groups.map(g => <MenuItem key={g.group_id} value={g as any} className="csp-item">
            {selectedGroups.includes(g) ? <CheckIcon />: <Icon />}
            {g.name}
          </MenuItem>)}
        </Select>
      </FormControl>
      <DialogContentText>
        Call this function to request notification permissions and subscribe the user these groups.
      </DialogContentText>
      <Copyable multiline>
        {getSubscribeCode(serviceId, vapidPublicKey, selectedGroups.map(g => g.group_id))}
      </Copyable>
      {/* <DialogContentText>
        You can also click the button below to subscribe your current browser to these groups.
      </DialogContentText>
      <DialogActions className="actions-left">
        <Button onClick={handleTestSubscribe}>
          Test on this Browser
        </Button>
      </DialogActions> */}
      <h3>Notify a Group</h3>
      <DialogContentText>Chosoe a group to send notifications to</DialogContentText>
      <FormControl fullWidth size="small" margin="dense">
        <InputLabel id="code-snippet-group-select">Group</InputLabel>
        <Select
          labelId="code-snippet-group-select"
          value={notifyGroup}
          label="Group"
          onChange={event => setNotifyGroup(event.target.value as ServiceGroup | '')}
        >
          {groups.map(g => <MenuItem key={g.group_id} value={g as any} className="csp-item">
            {g.name}
          </MenuItem>)}
        </Select>
      </FormControl>
      <DialogContentText>
        To send a notification to a group, make the following <code>POST</code> request:
      </DialogContentText>
      {notifyGroup && <Copyable multiline>{
        `curl ${config.NEXT_PUBLIC_APP_DOMAIN}/api/services/${serviceId}/groups/${notifyGroup.group_id}/notify` +
        `\n  -X POST \\` +
        `\n  -H 'Content-Type: application/json' \\` +
        `\n  --data-raw '{"apiKey":"YOUR_API_KEY","payload":"My first WebPush Notification"}'`
      }</Copyable>}
      <DialogContentText>
        Make sure to replace <code>YOUR_API_KEY</code> with your API key.
      </DialogContentText>
      <DialogActions>
        <Button onClick={handleClose}>Close</Button>
      </DialogActions>
    </Card>
  </Dialog>
}