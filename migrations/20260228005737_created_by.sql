alter table pontos add created_by varchar references users(id);
alter table voluntarios add created_by varchar references users(id);
alter table solicitacoes add created_by varchar references users(id);