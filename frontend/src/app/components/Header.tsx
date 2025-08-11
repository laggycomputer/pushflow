'use client';
import Image from 'next/image';
import './Header.scss';
import { Button, IconButton, MenuItem, Popover } from "@mui/material";
import Link from 'next/link';
import { useAppDispatch, useAppSelector } from '@/store/hooks';
import { openDialog } from '@/store/slices/dialogSlice';
import { DialogName } from '@/helpers/dialog';
import { User } from '@/types';

import AddIcon from '@mui/icons-material/Add';
import PersonIcon from '@mui/icons-material/Person';
import CodeIcon from '@mui/icons-material/Code';
import { useRef, useState } from 'react';
import LogoutIcon from '@mui/icons-material/Logout';

export default function Header ({ user }: { user: User | null }) {
  const dispatch = useAppDispatch()
  const service = useAppSelector(state => state.service)
  const hasService = !!service.currentServiceId
  const [profileMenuOpen, setProfileMenuOpen] = useState(false)

  const buttonRef = useRef<HTMLButtonElement | null>(null)

  const toggleAuth = async () => {
    if (!user) return location.href = '/api/login'
    setProfileMenuOpen(true)
  }

  const logout = async () => {
    if (!user) return
    await fetch('/api/logout', { method: 'POST', credentials: 'include' })
    location.href = '/'
  }

  const openCreateNewServiceForm = () => dispatch(openDialog(DialogName.NewServicePopup))

  const showSnippetPopup = () => {
    dispatch(openDialog(DialogName.ShowServiceCodeSnippetPopup))
  }

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
    {hasService && <Button startIcon={<CodeIcon />} onClick={showSnippetPopup}>
      <span>Integrate</span>
    </Button>}

    <IconButton onClick={toggleAuth} className="user-button" ref={buttonRef}>
      {user?.avatar
        ? <Image className="avatar" src={user.avatar} alt="Profile Picture" width={40} height={40}/>
        : <PersonIcon />
      }
    </IconButton>
      <Popover
        className="profile-menu"
        open={profileMenuOpen}
        anchorReference="anchorEl"
        anchorEl={buttonRef.current}
        anchorOrigin={{ vertical: 'bottom', horizontal: 'right' }}
        onClose={() => setProfileMenuOpen(false)}
      >
        <MenuItem className="csp-item" onClick={logout}><LogoutIcon/> Log Out</MenuItem>
      </Popover>
  </div>
}
