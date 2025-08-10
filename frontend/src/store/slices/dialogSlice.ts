import { DialogName } from '@/helpers/dialog';
import { createSlice, PayloadAction } from '@reduxjs/toolkit';

const dialogSlice = createSlice({
  name: 'dialog',
  initialState: {
    activeDialog: null as DialogName | null,
    key: ''
  },
  reducers: {
    closeDialog: state => {
      state.activeDialog = null
    },
    openDialog: (state, action: PayloadAction<DialogName>) => {
      state.activeDialog = action.payload;
      state.key = '';
    },
    openDialogWithKey: (state, action: PayloadAction<{ name: DialogName; key: string; }>) => {
      state.activeDialog = action.payload.name;
      state.key = action.payload.key;
    }
  }
});

export const { closeDialog, openDialog, openDialogWithKey } = dialogSlice.actions;

export default dialogSlice.reducer;
