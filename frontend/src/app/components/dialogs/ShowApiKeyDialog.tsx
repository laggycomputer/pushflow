import './ShowApiKeyDialog.scss';
import { ServiceApiKey } from "@/types";
import { Button, Dialog, DialogActions, DialogContentText, IconButton } from "@mui/material";
import Card, { CardHeader } from "../Card";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { setActiveDialog } from "@/store/slices/dialogSlice";

import ContentCopyIcon from '@mui/icons-material/ContentCopy';
import { DialogName } from '@/helpers/dialog';

interface ShowApiKeyDialogProps {
  apiKey?: ServiceApiKey;
}
export default function ShowApiKeyDialog ({ apiKey }: ShowApiKeyDialogProps) {
  const dispatch = useAppDispatch()
  const isOpen = useAppSelector(state => state.dialog.activeDialog === DialogName.ShowServiceApiKeyPopup)

  const handleClose = () => dispatch(setActiveDialog(null))
  
  if (!apiKey) return null

  const copyKey = () => {
    navigator.clipboard.writeText(apiKey.key_preview)
  }
  
  return <Dialog open={isOpen} onClose={handleClose} id="show-api-key-dialog">
    <Card>
      <CardHeader text="API Key Created" />
      <DialogContentText>
        Your new API Key, "{apiKey.name}", has been successfully created. Copy it now and store
        it somewhere safe, as it will not be shown again.
      </DialogContentText>
      <br />
      <div className="theme-emphasis-box copy-box">
        <span>
          {apiKey.key_preview}
        </span>
        <IconButton
          aria-label="Copy API Key"
          onClick={copyKey}
          className="theme-none"
        >
          <ContentCopyIcon />
        </IconButton>
      </div>
      <DialogActions>
        <Button onClick={handleClose}>Close</Button>
      </DialogActions>
    </Card>
  </Dialog>
}