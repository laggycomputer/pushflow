export interface User {
  user_id: string;
  avatar?: string;
}

export interface Service {
  service_id: string;
  owner_uid: string;
  name: string;
  vapid_public: string;
  // vapid private key will not be exposed
}

export interface ServiceSubscriber {
  service_id: string;
  subscriber_id: string;
  name?: string;
  email?: string;
  created_at: string;
  // endpoint will not be exposed
  // push_keys will not be exposed
  groups: string[];
}

export interface ServiceGroup {
  service_id: string;
  group_id: string;
  last_notified: string;
  created_at: string;
  name: string;
}

export interface ServiceApiKey {
  key_id: string;
  name: string;
  service_id: string;
  key_preview: string;
  last_used: string;
  created_at: string;
  scopes: { scope: string; group_id?: string; }[];
}
