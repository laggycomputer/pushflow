import { DialogName } from '@/helpers/dialog';
import { createSlice, PayloadAction } from '@reduxjs/toolkit';

const dialogSlice = createSlice({
  name: 'dialog',
  initialState: {
    activeDialog: null as DialogName | null
  },
  reducers: {
    setActiveDialog: (state, action: PayloadAction<DialogName | null>) => {
      state.activeDialog = action.payload;
    }
  }
});

export const { setActiveDialog } = dialogSlice.actions;

export default dialogSlice.reducer;
