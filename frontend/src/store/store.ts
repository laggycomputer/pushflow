import { configureStore } from '@reduxjs/toolkit';
import dialogReducer from './slices/dialogSlice';
import serviceReducer from './slices/serviceSlice';

export const store = configureStore({
  reducer: {
    dialog: dialogReducer,
    service: serviceReducer
  },
});

// Infer the `RootState` and `AppDispatch` types from the store itself
export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
