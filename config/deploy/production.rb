server '192.168.89.180', user: 'root', roles: %w{app}
#server '192.168.89.180', user: 'root', roles: %w{app}

server '192.168.89.180',
  user: 'root',
  roles: %w{app},
  ssh_options: {
    user: 'root', # overrides user setting above
    keys: %w(~/development/custom_image/id_rsa),
    forward_agent: true,
    auth_methods: %w(publickey password)
    # password: 'please use keys'
  }
