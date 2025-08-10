import { configureStore } from '@reduxjs/toolkit';
import dialogReducer from './slices/dialogSlice';

export const store = configureStore({
  reducer: {
    dialog: dialogReducer
  },
});

// Infer the `RootState` and `AppDispatch` types from the store itself
export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
