desc "Report Uptimes"
task :uptime do
  on roles(:all) do |host|
    #execute :any_command, "with args", :here, "and here"
    info "Host #{host} (#{host.roles.to_a.join(', ')}):\t#{capture(:uptime)}"
  end
end
