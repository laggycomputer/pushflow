'use client';
import Image from 'next/image';
import './Header.scss';
import { Button, IconButton } from "@mui/material";
import AddIcon from '@mui/icons-material/Add';
import PersonIcon from '@mui/icons-material/Person';
import Link from 'next/link';
import { useAppDispatch } from '@/store/hooks';
import { setActiveDialog } from '@/store/slices/dialogSlice';
import { DialogName } from '@/helpers/dialog';
import { User } from '@/types';

export default function Header ({ user }: { user: User | null }) {
  const dispatch = useAppDispatch()

  const toggleAuth = async () => {
    if (!user) return location.href = '/api/login'
    await fetch('/api/logout', { method: 'POST', credentials: 'include' })
    location.href = '/'
  }

  const openCreateNewServiceForm = () => dispatch(setActiveDialog(DialogName.NewServicePopup))
  console.log(user)

  return <div className="app-header">
    <Link href="/">
      <Image src="/logo.png" width={48} height={48} alt="Icon" />
      <h1>PushFlow</h1>
    </Link>

    <Button startIcon={<AddIcon />} onClick={openCreateNewServiceForm}>
      <span>New Service</span>
    </Button>
    <IconButton onClick={toggleAuth} className="user-button">
      {user?.avatar
        ? <Image className="avatar" src={user.avatar} alt="Profile Picture" width={40} height={40}/>
        : <PersonIcon />
      }
    </IconButton>
  </div>
}
