using Dapper;
using System.Threading;
using System.Threading.Tasks;
using Calendar.Server.Application.Dtos.Authentication;
using Calendar.Server.Application.Infrastructure;
using MediatR;

namespace Calendar.Server.Application.Domain.Authentication.Commands
{
    public class LoginCommand : IRequest<bool>
    {
        public LoginDto LoginDto { get; set; }
    }

    public class LoginHandler : BaseHandler, IRequestHandler<LoginCommand, bool>
    {
        public LoginHandler(ISqlSettings settings) : base(settings) { }

        public Task<bool> Handle(LoginCommand command, CancellationToken cancellationToken) =>
            _db.QuerySingleAsync<bool>("SELECT COUNT(*) FROM Password WHERE Hash = @Hash", new { Hash = ComputeSHA256Hash(command.LoginDto.Password) });
    }
}
