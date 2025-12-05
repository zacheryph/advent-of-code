# frozen_string_literal: true

challenges_dir = File.join(File.dirname(__FILE__), "..", "challenges")

Dir[File.join(challenges_dir, "shared", "**", "*.rb")].each do |file|
  require file
end

Dir[File.join(challenges_dir, "20*", "**", "*.rb")].each do |file|
  require file
end

RSpec.configure do |config|

end
