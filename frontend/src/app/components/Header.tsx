'use client';
import Image from 'next/image';
import './Header.scss';
import { Button, IconButton } from "@mui/material";
import Link from 'next/link';
import { useAppDispatch, useAppSelector } from '@/store/hooks';
import { openDialog } from '@/store/slices/dialogSlice';
import { DialogName } from '@/helpers/dialog';
import { User } from '@/types';

import AddIcon from '@mui/icons-material/Add';
import PersonIcon from '@mui/icons-material/Person';
import PersonAddIcon from '@mui/icons-material/PersonAdd';

export default function Header ({ user }: { user: User | null }) {
  const dispatch = useAppDispatch()
  const service = useAppSelector(state => state.service)
  const hasService = !!service.currentServiceId

  const toggleAuth = async () => {
    if (!user) return location.href = '/api/login'
    await fetch('/api/logout', { method: 'POST', credentials: 'include' })
    location.href = '/'
  }

  const openCreateNewServiceForm = () => dispatch(openDialog(DialogName.NewServicePopup))

  return <div className="app-header">
    <Link href="/">
      <Image src="/logo.png" width={48} height={48} alt="Icon" />
      {!hasService && <h1>PushFlow</h1>}
    </Link>
    {hasService && <h1>{service.name}</h1>}

    {/** @todo refactor */}
    {!hasService && <Button startIcon={<AddIcon />} onClick={openCreateNewServiceForm}>
      <span>New Service</span>
    </Button>}
    {hasService && <Button startIcon={<PersonAddIcon />}>
      <span>Share</span>
    </Button>}

    <IconButton onClick={toggleAuth} className="user-button">
      {user?.avatar
        ? <Image className="avatar" src={user.avatar} alt="Profile Picture" width={40} height={40}/>
        : <PersonIcon />
      }
    </IconButton>
  </div>
}
