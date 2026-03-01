CREATE INDEX idx_voluntarios_app_id_created_at ON voluntarios (app_id, created_at DESC);
CREATE INDEX idx_solicitacoes_app_id_created_at ON solicitacoes (app_id, created_at DESC);

ALTER TABLE votos_pontos
ADD PRIMARY KEY (app_id, ponto_id, user_id);

ALTER TABLE itens_pontos
ADD PRIMARY KEY (app_id, item);

