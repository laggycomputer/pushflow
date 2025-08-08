export interface User {
  user_id: string;
  google_id: string;
  profile_url?: string;
}

export interface Service {
  service_id: string;
  owner_uid: string;
  name: string;
  // vapid keys will not be exposed
}

export interface ServiceSubscriber {
  service_id: string;
  subscriber_id: string;
  name?: string;
  email?: string;
  // endpoint will not be exposed
  // push_keys will not be exposed
  groups: string[];
}

export interface ServiceGroup {
  service_id: string;
  group_id: string;
  last_notified: string;
  created_at: string;
}

export interface ServiceApiKey {
  service_id: string;
  key_uuid: string;
  last_used: string;
  created_at: string;
  scopes: string[]; // may expand to { role: '', group: '' } later
}
