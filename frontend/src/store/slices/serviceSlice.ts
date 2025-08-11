import { ServiceApiKey, ServiceGroup, ServiceSubscriber } from '@/types';
import { createSlice, PayloadAction } from '@reduxjs/toolkit';

interface ServiceSliceState {
  currentServiceId: string | null,
  name: string;
  vapidPublic: string;
  apiKeys: ServiceApiKey[];
  groups: ServiceGroup[];
  subscribers: ServiceSubscriber[];
}

const serviceSlice = createSlice({
  name: 'serviceApiKeysSlice',
  initialState: {
    currentServiceId: null,
    name: '',
    vapidPublic: '',
    apiKeys: [],
    groups: [],
    subscribers: []
  } as ServiceSliceState,
  reducers: {
    resetState: (state) => {
      state.currentServiceId = null
      state.name = ''
      state.vapidPublic = ''
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
    editApiKey: (state, action: PayloadAction<ServiceApiKey>) => {
      const apiKey = state.apiKeys.find(key => key.key_preview === action.payload.key_preview)
      if (apiKey) Object.assign(apiKey, action.payload)
    },
    addGroup: (state, action: PayloadAction<ServiceGroup>) => {
      state.groups.push(action.payload)
    },
    removeGroup: (state, action: PayloadAction<ServiceGroup>) => {
      const index = state.groups.findIndex(g => g.group_id === action.payload.group_id)
      if (index > -1) state.groups.splice(index, 1)
    },
    editGroup: (state, action: PayloadAction<ServiceGroup>) => {
      const group = state.groups.find(g => g.group_id === action.payload.group_id)
      if (group) Object.assign(group, action.payload)
    },
    removeSubscriber: (state, action: PayloadAction<ServiceSubscriber>) => {
      const index = state.subscribers.findIndex(s => s.subscriber_id === action.payload.subscriber_id)
      if (index > -1) state.subscribers.splice(index, 1)
    },
    editSubscriber: (state, action: PayloadAction<ServiceSubscriber>) => {
      const sub = state.subscribers.findIndex(s => s.subscriber_id === action.payload.subscriber_id)
      if (sub) Object.assign(sub, action.payload)
    }
  }
});

export const {
  resetState,
  setInitialData,
  prependApiKey,
  removeApiKey,
  editApiKey,
  addGroup,
  removeGroup,
  editGroup,
  removeSubscriber,
  editSubscriber
} = serviceSlice.actions;

export default serviceSlice.reducer;
