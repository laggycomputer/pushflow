import Image from 'next/image';
import './Header.scss';
import { Button, IconButton } from "@mui/material";
import AddIcon from '@mui/icons-material/Add';

export default function Header () {
  return <div className="app-header">
    <Image src="/flow-base-background.png" width={48} height={48} alt="Icon" />
    <h1>PushFlow</h1>

    <Button startIcon={<AddIcon />}>
      <span>New Service</span>
    </Button>
    <IconButton>
      <AddIcon />
    </IconButton>
  </div>
}
