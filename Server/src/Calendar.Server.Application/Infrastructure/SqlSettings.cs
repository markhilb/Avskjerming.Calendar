using System;

namespace Calendar.Server.Application.Infrastructure
{
    public class SqlSettings : ISqlSettings
    {
    }

    public class ISqlSettings
    {
        public string DatabaseName { get; set; }
        public string Host { get; set; }
        public string Username { get; set; }
        public string Password { get; } = Environment.GetEnvironmentVariable("DATABASE_PASSWORD");
        public int Port { get; set; }

        public string ConnectionString =>
            $"Server={Host},{Port}; Database={DatabaseName}; User Id={Username}; Password={Password}; MultipleActiveResultSets=True";
    }
}
