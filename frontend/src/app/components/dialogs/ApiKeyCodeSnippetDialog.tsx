import './ApiKeyCodeSnippetDialog.scss'
import { Button, Dialog, DialogActions, DialogContentText } from "@mui/material";
import Card, { CardHeader } from "../Card";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { closeDialog } from "@/store/slices/dialogSlice";

import { DialogName } from '@/helpers/dialog';
import { getServiceWorkerBaseCode, getServiceWorkerRegistrationCode, getSubscribeCode } from "@/helpers/snippet";
import Copyable from "../Copyable";
import config from '@/env';

export default function ApiKeyCodeSnippetDialog () {
  const dispatch = useAppDispatch()
  const isOpen = useAppSelector(state => state.dialog.activeDialog === DialogName.ShowServiceApiKeySnippetPopup)
  const serviceId = useAppSelector(state => state.service.currentServiceId)
  const serviceName = useAppSelector(state => state.service.name)
  const vapidPublicKey = useAppSelector(state => state.service.vapidPublic)
  
  if (!serviceId) return null

  const handleClose = () => dispatch(closeDialog())

  
  return <Dialog open={isOpen} onClose={handleClose} id="api-key-snippet-popup">
    <Card className="list-container">
      <CardHeader text="Integrate API Key" />
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
      </Copyable>
      <br />
      <DialogContentText>
        Call this function to request notification permissions and subscribe the user to &quot;{serviceName}&quot;.
      </DialogContentText>
      <Copyable multiline>
        {getSubscribeCode(serviceId, 'YOUR_API_KEY', vapidPublicKey)}
      </Copyable>
      <DialogContentText>
        To send a notification to a group, make the following <code>POST</code> request:
      </DialogContentText>
      <Copyable multiline>{
        `curl ${config.NEXT_PUBLIC_APP_DOMAIN}/api/services/${serviceId}/groups/GROUP_ID/notify` +
        `\n  -X POST \\` +
        `\n  -H 'Content-Type: application/json' \\` +
        `\n  --data-raw '{"apiKey":"YOUR_API_KEY","payload":"My first WebPush Notification"}'`
      }</Copyable>
      <DialogContentText>
        Make sure to replace <code>GROUP_ID</code> with the ID of the group you would like to notify and{' '}
        <code>YOUR_API_KEY</code> with your API key.
      </DialogContentText>
      <DialogActions>
        <Button onClick={handleClose}>Close</Button>
      </DialogActions>
    </Card>
  </Dialog>
}