using Dapper;
using System.Threading;
using System.Threading.Tasks;
using Calendar.Server.Application.Dtos.Authentication;
using Calendar.Server.Application.Infrastructure;
using MediatR;

namespace Calendar.Server.Application.Domain.Authentication.Commands
{
    public class ChangePasswordCommand : IRequest<bool>
    {
        public ChangePasswordDto ChangePasswordDto { get; set; }
    }

    public class ChangePasswordHandler : BaseHandler, IRequestHandler<ChangePasswordCommand, bool>
    {
        protected readonly IMediator _mediator;

        public ChangePasswordHandler(ISqlSettings settings, IMediator mediator) : base(settings) =>
            _mediator = mediator;

        public async Task<bool> Handle(ChangePasswordCommand command, CancellationToken cancellationToken)
        {
            var ok = await _mediator.Send(new LoginCommand { LoginDto = new LoginDto { Password = command.ChangePasswordDto.OldPassword } }, cancellationToken);
            if (!ok)
                return false;

            var updatedRows = await _db.ExecuteAsync("UPDATE Password SET Hash = @Hash", new { Hash = ComputeSHA256Hash(command.ChangePasswordDto.NewPassword) });
            return updatedRows == 1;
        }
    }
}
