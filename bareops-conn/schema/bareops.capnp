@0xd18e42b92efe525b;

struct PluginOption {
  name @0 :Text;
  value @1 :Text;
}

struct Plugin {
  name @0 :Text;
  options @1 :List(PluginOption);
}

struct Tag {
  name @0 :Text;
}

struct Task {
  name @0 :Text;
  plugins @1 :List(Plugin);
  tags @2 :List(Tag);
}

interface TaskClient {}
