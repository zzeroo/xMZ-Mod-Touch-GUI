namespace :deploy do

  desc "Compile xMZ-Mod-Touch-Gui"
  task :compile do
    on roles(:app) do
      execute "cd #{fetch(:deploy_to)}/current; sh ./autogen.sh"
      execute "cd #{fetch(:deploy_to)}/current; sh ./configure \
        --prefix=/usr"
      execute "cd #{fetch(:deploy_to)}/current; make"
    end
  end

  desc "Install xMZ-Mod-Touch-Gui"
  task :make_install do
    on roles(:app) do
      execute "cd #{fetch(:deploy_to)}/current; make install"
    end
  end

  desc "Start installation"
  task :build do
    invoke "deploy:compile"
    #invoke "xmz_mod_touch_gui:make_tests"
    invoke "deploy:make_install"
  end


  after :build, :restart do
    on roles(:app) do
      execute "service xmz restart"
    end
  end
end

