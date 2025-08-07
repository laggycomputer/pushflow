'use client';
import Image from 'next/image';
import './Header.scss';
import { Button, IconButton } from "@mui/material";
import AddIcon from '@mui/icons-material/Add';
import PersonIcon from '@mui/icons-material/Person';

export default function Header () {
  const initLogin = async () => {
    location.href = '/api/login'
    /** @todo create /api/user endpoint to get user data given a session jwt (issued by oauth callback?) */
  }


  return <div className="app-header">
    <Image src="/flow-base-background.png" width={48} height={48} alt="Icon" />
    <h1>PushFlow</h1>

    <Button startIcon={<AddIcon />}>
      <span>New Service</span>
    </Button>
    <IconButton onClick={initLogin}>
      <PersonIcon />
    </IconButton>
  </div>
}
