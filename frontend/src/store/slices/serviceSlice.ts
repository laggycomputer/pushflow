import { ServiceApiKey, ServiceGroup, ServiceSubscriber } from '@/types';
import { createSlice, PayloadAction } from '@reduxjs/toolkit';

interface ServiceSliceState {
  currentServiceId: string | null,
  name: string;
  apiKeys: ServiceApiKey[];
  groups: ServiceGroup[];
  subscribers: ServiceSubscriber[];
}

const serviceSlice = createSlice({
  name: 'serviceApiKeysSlice',
  initialState: {
    currentServiceId: null,
    name: '',
    apiKeys: [],
    groups: [],
    subscribers: []
  } as ServiceSliceState,
  reducers: {
    resetState: (state) => {
      state.currentServiceId = null
      state.name = ''
      state.apiKeys = []
      state.groups = []
      state.subscribers = []
    },
    setInitialData: (state, action: PayloadAction<ServiceSliceState>) => {
      Object.assign(state, action.payload)
    },
    prependApiKey: (state, action: PayloadAction<ServiceApiKey>) => {
      state.apiKeys.unshift(action.payload)
    },
    removeApiKey: (state, action: PayloadAction<ServiceApiKey>) => {
      const index = state.apiKeys.findIndex(key => key.key_preview === action.payload.key_preview)
      if (index > -1) state.apiKeys.splice(index, 1)
    },
    addGroup: (state, action: PayloadAction<ServiceGroup>) => {
      state.groups.push(action.payload)
    },
    removeGroup: (state, action: PayloadAction<ServiceGroup>) => {
      const index = state.groups.findIndex(g => g.group_id === action.payload.group_id)
      if (index > -1) state.groups.splice(index, 1)
    }
  }
});

export const { resetState, setInitialData, prependApiKey, removeApiKey, addGroup, removeGroup } = serviceSlice.actions;

export default serviceSlice.reducer;
