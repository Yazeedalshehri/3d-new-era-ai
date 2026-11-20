//! Arabic: the fifth column of the interface, kept in a file of its own so
//! the table the other languages share stays as it is.
//!
//! The text here is in logical order, as it is typed; [`visual`] prepares it
//! for egui, which shapes an Arabic run right to left but has no bidi of its
//! own.

use std::collections::HashMap;
use std::sync::OnceLock;

/// Portuguese text of the interface → Arabic.
const TABLE: &[(&str, &str)] = &[
    (" · arco", " · قوس"),
    (" (cópia)", " (نسخة)"),
    ("Nova casa", "منزل جديد"),
    ("Versão {}", "الإصدار {}"),
    (
        "A tarefa termina sozinha em segundo plano; o resultado é descartado.",
        "ستنتهي المهمة وحدها في الخلفية، وستُتجاهل نتيجتها.",
    ),
    ("Abrindo imagens e modelos", "جارٍ فتح الصور والنماذج"),
    ("Abrindo projeto", "جارٍ فتح المشروع"),
    ("Desenhando a planta", "جارٍ رسم المخطط"),
    ("Exportando", "جارٍ التصدير"),
    ("Extraindo imagens e modelos", "جارٍ استخراج الصور والنماذج"),
    ("Forçar e encerrar a espera", "إيقاف الانتظار"),
    ("Gravando o projeto", "جارٍ كتابة المشروع"),
    ("Guardando imagens e modelos", "جارٍ حفظ الصور والنماذج"),
    ("Importando modelo", "جارٍ استيراد النموذج"),
    ("Lendo o arquivo", "جارٍ قراءة الملف"),
    ("Lendo o projeto", "جارٍ قراءة المشروع"),
    ("Montando o modelo 3D", "جارٍ بناء النموذج ثلاثي الأبعاد"),
    ("Parando…", "جارٍ الإيقاف…"),
    ("Salvando projeto", "جارٍ حفظ المشروع"),
    ("Trabalhando…", "جارٍ العمل…"),
    (
        "⚠ Espera encerrada — a tarefa termina em segundo plano.",
        "⚠ توقف الانتظار — ستنتهي المهمة في الخلفية.",
    ),
    ("Ajustar ao telhado", "ملاءمة مع السقف"),
    ("Código de obras", "كود البناء"),
    (
        "Embutir peça no móvel selecionado",
        "تضمين القطعة في النجارة المحددة",
    ),
    ("Embutido:", "مُضمّن:"),
    ("Armários na parede", "خزائن على الجدار"),
    ("Armários na parede…", "خزائن على الجدار…"),
    ("Balcões", "خزائن سفلية"),
    ("Aéreos", "خزائن علوية"),
    (
        "Entre a norma e a lei do município, prevalece o mais restritivo.",
        "بين المعيار ونظام البلدية، يُطبَّق الأكثر صرامة.",
    ),
    ("Fontes desta revisão:", "مصادر هذه المراجعة:"),
    ("Não informado", "غير محدد"),
    (
        "Referências brasileiras onde existe norma; áreas e janelas variam com o código de obras do município.",
        "معايير برازيلية حيث يوجد معيار؛ المساحات والنوافذ تختلف حسب كود البناء في البلدية.",
    ),
    ("Torres / guarda-roupa", "خزانة طويلة / دولاب"),
    ("Pia centrada em", "مركز المغسلة عند"),
    ("Cooktop centrado em", "مركز الموقد عند"),
    ("descreve", "يصف"),
    ("do início da parede", "من بداية الجدار"),
    ("Bancada de pedra por cima", "سطح حجري في الأعلى"),
    ("Frentes", "الواجهات"),
    ("Gaveteiros", "وحدات أدراج"),
    ("automático", "تلقائي"),
    (
        "Mede o trecho livre entre cantos, portas, janelas e eletrodomésticos e divide em módulos sem sobras; substitui os armários que já estão nessa parede.",
        "يقيس المسافة الحرة بين الزوايا والأبواب والنوافذ والأجهزة ويقسمها إلى وحدات بلا فائض؛ ويستبدل الخزائن الموجودة على ذلك الجدار.",
    ),
    ("Pré-visualizar", "معاينة"),
    ("Criar armários", "بناء الخزائن"),
    ("Criado (Ctrl+Z desfaz):", "تم البناء (Ctrl+Z للتراجع):"),
    ("Prévia:", "معاينة:"),
    ("doutrina", "مبدأ"),
    ("mede", "يقيس"),
    ("obriga", "يُلزم"),
    ("portas", "أبواب"),
    ("gaveteiro", "أدراج"),
    ("porta-temperos", "رف بهارات منزلق"),
    ("canto cego", "زاوية عمياء"),
    ("cabideiro", "معلّقة"),
    ("referencia", "مراجع"),
    ("sobre a geladeira", "فوق الثلاجة"),
    ("pia", "مغسلة"),
    ("tamponamento", "حشوة"),
    ("bancada", "سطح العمل"),
    ("Ergonomia", "بيئة العمل"),
    ("Ergonomia…", "بيئة العمل…"),
    ("Moradores", "الساكنون"),
    ("Crianças", "أطفال"),
    ("Idosos", "كبار السن"),
    ("Altura de quem cozinha", "طول الطبّاخ"),
    (
        "Alguém usa cadeira de rodas (NBR 9050)",
        "أحد الساكنين يستخدم كرسيًا متحركًا (NBR 9050)",
    ),
    ("Incluir elétrica na nota", "تضمين الكهرباء في التقييم"),
    ("Incluir hidráulica na nota", "تضمين السباكة في التقييم"),
    (
        "Pendências fora do escopo continuam visíveis. A nota não certifica o projeto.",
        "الملاحظات خارج النطاق تبقى ظاهرة. التقييم لا يعتمد المشروع رسميًا.",
    ),
    ("Fora do escopo da nota", "خارج نطاق التقييم"),
    ("nota de habitabilidade", "تقييم صلاحية السكن"),
    ("lugares para dormir", "أسرّة"),
    ("banheiros", "دورات مياه"),
    ("à mesa", "على الطاولة"),
    ("na sala", "في غرفة المعيشة"),
    ("de guarda-roupa", "من الدولاب"),
    (
        "Nada a apontar para esses moradores.",
        "لا توجد ملاحظات لهؤلاء الساكنين.",
    ),
    ("Selecionar na planta", "تحديد على المخطط"),
    ("Aplicar correção", "تطبيق الإصلاح"),
    (
        "Correção aplicada (Ctrl+Z desfaz).",
        "تم تطبيق الإصلاح (Ctrl+Z للتراجع).",
    ),
    (
        "Referências: NBR 9050, NBR 15575-1 e IBGE; áreas e janelas variam com o código de obras do município.",
        "المراجع: NBR 9050 و NBR 15575-1 و IBGE؛ المساحات والنوافذ تختلف حسب كود البناء في البلدية.",
    ),
    (" · luz", " · إضاءة"),
    ("(cópia)", "(نسخة)"),
    ("Abrir", "فتح"),
    ("Abrir (Ctrl+O)", "فتح (Ctrl+O)"),
    ("Abrir link", "فتح الرابط"),
    ("Abrir…", "فتح…"),
    ("Adicionar texto", "إضافة نص"),
    (
        "Adicionar um andar acima do mais alto",
        "إضافة طابق فوق أعلى طابق",
    ),
    ("Afastamento", "إزاحة"),
    ("Afastar (Ctrl -)", "تصغير (Ctrl -)"),
    ("Ajuda", "مساعدة"),
    ("Quadro de cargas e NBR 5410", "جدول الأحمال و NBR 5410"),
    (
        "Nenhum circuito: atribua os pontos a circuitos (MCP electrical assign).",
        "لا توجد دوائر: اربط النقاط بالدوائر (MCP electrical assign).",
    ),
    ("Circuito", "الدائرة"),
    ("Pontos", "النقاط"),
    ("VA", "VA"),
    ("V", "V"),
    ("A", "A"),
    ("Fio mm²", "سلك مم²"),
    ("Disjuntor", "قاطع"),
    ("DR", "قاطع تسرب أرضي"),
    ("Total instalado:", "الإجمالي المركّب:"),
    ("Nada a apontar.", "لا توجد ملاحظات."),
    ("Camadas", "الطبقات"),
    ("Iluminação", "الإضاءة"),
    ("Eletrodomésticos", "الأجهزة"),
    ("Marcenaria", "النجارة"),
    (
        "Mostrar ou esconder na planta e no 3D",
        "إظهار أو إخفاء في المخطط وفي العرض ثلاثي الأبعاد",
    ),
    ("Mostrar tudo no 3D", "إظهار كل شيء في العرض ثلاثي الأبعاد"),
    (
        "Ligado, o 3D mostra a obra inteira; desligado, esconde o que a planta esconde",
        "عند التفعيل يظهر المبنى كاملًا في العرض ثلاثي الأبعاد؛ وعند الإيقاف يُخفى ما يخفيه المخطط",
    ),
    ("Enviar relatórios de erro", "إرسال تقارير الأخطاء"),
    (
        "Falhas e notas de uso vão para os desenvolvedores, sem o seu projeto, sem IP e sem nome da máquina.",
        "تُرسل الأعطال وملاحظات الاستخدام إلى المطوّرين، بدون مشروعك أو عنوان IP أو اسم جهازك.",
    ),
    ("Ajustes…", "الإعدادات…"),
    ("Alinhamento", "المحاذاة"),
    ("Altura", "الارتفاع"),
    ("Andar", "الطابق"),
    ("Aproximar (Ctrl +)", "تكبير (Ctrl +)"),
    ("Apoiar o projeto", "ادعم المشروع"),
    ("Apoiar no Buy Me a Coffee", "ادعم عبر Buy Me a Coffee"),
    ("Abrir o GitHub", "فتح GitHub"),
    ("Agora não", "ليس الآن"),
    ("Nova versão {} disponível", "يتوفر إصدار جديد {}"),
    (
        "Muita gente usa o 3D New Era AI todos os dias, mas quase ninguém apoia o projeto.",
        "كثيرون يستخدمون 3D New Era AI يوميًا، لكن قلّة قليلة تدعم المشروع.",
    ),
    (
        "As IAs que usam a ferramenta enviam sozinhas relatos de problemas e sugestões de melhoria, e eles chegam em grande número. Quero corrigir todos, mas me falta tempo e dinheiro para pagar as ferramentas de IA e os tokens que esse trabalho exige.",
        "أدوات الذكاء الاصطناعي التي تستخدم البرنامج ترسل تلقائيًا بلاغات عن المشكلات واقتراحات للتحسين، وتصل بأعداد كبيرة. أريد إصلاحها كلها، لكن ينقصني الوقت والمال لدفع تكاليف أدوات الذكاء الاصطناعي والتوكنات التي يتطلبها هذا العمل.",
    ),
    (
        "Se a ferramenta é útil para você, considere apoiar no Buy Me a Coffee.",
        "إذا كان البرنامج مفيدًا لك، فكّر في دعمه عبر Buy Me a Coffee.",
    ),
    (
        "Também dá para ajudar com código: abra issues e pull requests no GitHub. Todos serão avaliados, e o que for aceito entra na próxima versão.",
        "يمكنك المساعدة بالبرمجة أيضًا: افتح Issues و Pull Requests في GitHub. ستُراجع جميعها، وما يُقبل يدخل في الإصدار التالي.",
    ),
    ("Arco", "قوس"),
    ("Arquitetura", "معماري"),
    ("Arquivo", "ملف"),
    ("Arraste para mover a vista", "اسحب لتحريك العرض"),
    (
        "Arraste: girar · Shift/botão do meio: mover · Scroll: zoom · F: enquadrar",
        "اسحب: دوران · Shift/الزر الأوسط: تحريك · العجلة: تكبير · F: ملاءمة",
    ),
    ("Atalhos e ferramentas", "الاختصارات والأدوات"),
    (
        "Automático (símbolos e vista de cima dos modelos)",
        "تلقائي (رموز، ومنظر علوي للنماذج)",
    ),
    ("Autor", "المؤلف"),
    ("Boa", "جيد"),
    ("Buscar: cama, janela, sofá…", "ابحث: سرير، نافذة، كنب…"),
    ("Calibrar e posicionar", "المعايرة والتموضع"),
    ("Calibrar escala da imagem", "معايرة مقياس الصورة"),
    ("Cancelar", "إلغاء"),
    ("Casa e bússola", "المنزل والبوصلة"),
    ("Casa e bússola…", "المنزل والبوصلة…"),
    ("Catálogo", "الكتالوج"),
    ("Catálogo de origem", "الكتالوج المصدر"),
    ("Centro", "المركز"),
    ("Centro (x, y)", "المركز (x, y)"),
    (
        "Clique dois pontos de medida conhecida · arraste para posicionar a imagem",
        "انقر نقطتين بينهما مسافة معروفة · اسحب لتحريك الصورة",
    ),
    (
        "Clique e depois clique na planta para posicionar",
        "انقر، ثم انقر على المخطط للوضع",
    ),
    (
        "Clique encadeia paredes · digite o comprimento + Enter · Shift desliga o ímã · duplo clique encerra",
        "النقرات تصل الجدران · اكتب الطول ثم Enter · Shift يلغي الالتصاق · النقر المزدوج ينهي",
    ),
    (
        "Clique início e fim, mova para afastar e clique · duplo clique numa parede cota a parede",
        "انقر البداية والنهاية، حرّك للإزاحة وانقر · النقر المزدوج على جدار يضع أبعاده",
    ),
    ("Clique onde o texto deve ficar", "انقر حيث يوضع النص"),
    (
        "Clique os cantos e duplo clique fecha · duplo clique dentro de paredes detecta o cômodo",
        "انقر الزوايا والنقر المزدوج يغلق · النقر المزدوج داخل الجدران يكتشف الغرفة",
    ),
    (
        "Clique os pontos e duplo clique encerra · na Elétrica/Hidráulica a linha vira eletroduto/tubulação",
        "انقر النقاط والنقر المزدوج ينهي · في الكهرباء/السباكة يصبح الخط مواسير/أنابيب",
    ),
    (
        "Clique para posicionar · portas e janelas encaixam na parede mais próxima · Esc cancela",
        "انقر للوضع · الأبواب والنوافذ تلتصق بأقرب جدار · Esc للإلغاء",
    ),
    (
        "Clique seleciona · Ctrl+clique soma · arraste move · alças editam · duplo clique modifica",
        "النقر يحدد · Ctrl+نقر يضيف · السحب ينقل · المقابض تعدّل · النقر المزدوج يعدّل الخصائص",
    ),
    ("Colar", "لصق"),
    (
        "Comece desenhando paredes (W), importe uma planta como imagem de fundo, ou peça para a IA via MCP.",
        "ابدأ برسم الجدران (W)، أو استورد مخططًا كصورة خلفية، أو اطلب من الذكاء الاصطناعي عبر MCP.",
    ),
    ("Comparar", "مقارنة"),
    ("Comparar as versões", "مقارنة الإصدارات"),
    ("Comparar versões", "مقارنة الإصدارات"),
    ("Comprimento", "الطول"),
    (
        "Comprimento exato da parede sendo desenhada",
        "الطول الدقيق للجدار الجاري رسمه",
    ),
    ("Contorno", "المحيط"),
    ("Contínuo", "متصل"),
    ("Copiar", "نسخ"),
    (
        "Copiar · Recortar · Colar · Duplicar",
        "نسخ · قص · لصق · تكرار",
    ),
    ("Cor", "اللون"),
    ("Cotar paredes selecionadas", "وضع أبعاد الجدران المحددة"),
    ("Cotas automáticas (engenharia)", "أبعاد تلقائية (هندسية)"),
    ("Criar cotas", "إنشاء الأبعاد"),
    ("Criar cômodos", "إنشاء الغرف"),
    ("Criar foto", "إنشاء صورة"),
    ("Criar foto…", "إنشاء صورة…"),
    ("Criar vídeo", "إنشاء فيديو"),
    ("Criar vídeo…", "إنشاء فيديو…"),
    ("Pontos do caminho", "نقاط المسار"),
    ("Adicionar ponto atual", "إضافة النقطة الحالية"),
    (
        "Ative o visitante na vista 3D",
        "تشغيل وضع الزائر في العرض ثلاثي الأبعاد",
    ),
    ("Órbita aérea", "دوران جوي"),
    ("Limpar", "مسح"),
    ("Quadros por segundo", "إطارات في الثانية"),
    ("Velocidade", "السرعة"),
    ("Gerar vídeo…", "تصيير الفيديو…"),
    ("Vídeo salvo em", "حُفظ الفيديو في"),
    ("quadros", "إطارات"),
    ("Sol pela bússola e hora", "الشمس حسب البوصلة والوقت"),
    ("Vista 3D", "العرض ثلاثي الأبعاد"),
    ("Mostrar em 3D", "إظهار في 3D"),
    ("Inclinação", "الميل"),
    ("Deitado", "أفقي"),
    ("Em pé", "قائم"),
    ("Editor no navegador", "المحرر في المتصفح"),
    (
        "Disponível no aplicativo para desktop.",
        "متاح في تطبيق سطح المكتب.",
    ),
    (
        "No navegador, exporte em .glb.",
        "في المتصفح، صدّر بصيغة .glb.",
    ),
    ("Nenhum plugin instalado", "لا توجد إضافات مثبتة"),
    ("Pastas", "المجلدات"),
    ("Rodando", "قيد التشغيل"),
    ("concluído", "تم"),
    ("Acompanha as paredes", "يتبع الجدران"),
    (
        "O contorno é detectado pelas paredes e divisores e se ajusta quando eles mudam",
        "يُكتشف المحيط من الجدران والفواصل ويتكيف عند تغييرها",
    ),
    ("Ambientes", "الغرف"),
    ("Divisor de ambiente", "فاصل غرف"),
    (
        "Separa ambientes sem parede (ex.: sala e jantar integrados)",
        "يفصل الغرف بدون جدار (مثل صالة وطعام مفتوحين)",
    ),
    ("Escala vertical", "المقياس الرأسي"),
    ("Diferente", "مختلف"),
    ("Criar paredes", "إنشاء جدران"),
    ("Curva suave", "منحنى ناعم"),
    ("Cômodos", "الغرف"),
    (
        "Da sua biblioteca de modelos. Clique para pôr no centro da planta",
        "من مكتبة نماذجك. انقر لوضعه في منتصف المخطط",
    ),
    ("Descartar", "تجاهل"),
    ("Descrição", "الوصف"),
    ("Desenhar linhas", "رسم خطوط"),
    ("Desfazer", "تراجع"),
    ("Desfazer (Ctrl+Z)", "تراجع (Ctrl+Z)"),
    (
        "Desfazer · Refazer (inclusive o que a IA fez)",
        "تراجع · إعادة (يشمل ما فعله الذكاء الاصطناعي)",
    ),
    (
        "Desliga o ímã (ângulos de 15°, pontos e grade)",
        "يلغي الالتصاق (زوايا 15°، النقاط والشبكة)",
    ),
    (
        "Detalhes: marca, modelo e link",
        "التفاصيل: العلامة التجارية والطراز والرابط",
    ),
    ("Digitar número + Enter", "اكتب الرقم ثم Enter"),
    ("Direita", "يمين"),
    ("Disco", "قرص"),
    ("Distância real", "المسافة الحقيقية"),
    ("Dividir parede ao meio", "تقسيم الجدار إلى نصفين"),
    ("Diâmetro", "القطر"),
    ("Dobradiça à direita", "المفصلة على اليمين"),
    ("Duplicar", "تكرار"),
    ("Duplicar versão atual", "تكرار الإصدار الحالي"),
    (
        "Duplicar versão · Próxima versão (guias)",
        "تكرار الإصدار · الإصدار التالي (التبويبات)",
    ),
    ("Duplo clique", "نقر مزدوج"),
    ("Editar", "تحرير"),
    ("Editar andar…", "تحرير الطابق…"),
    ("Elevação", "المنسوب"),
    ("Elevação do piso", "منسوب الأرضية"),
    ("Elétrica", "الكهرباء"),
    (
        "Elétrica (tomadas, luz, cabos)",
        "الكهرباء (مقابس، إنارة، كابلات)",
    ),
    (
        "Encerra paredes · fecha/detecta cômodo · cota parede · modifica",
        "ينهي الجدران · يغلق/يكتشف الغرفة · يضع أبعاد الجدار · يعدّل",
    ),
    ("Enquadrar (Ctrl+0)", "ملاءمة (Ctrl+0)"),
    ("Enquadrar 3D", "ملاءمة 3D"),
    ("Enquadrar planta", "ملاءمة المخطط"),
    ("Escala", "المقياس"),
    (
        "Escala calibrada. Arraste para posicionar a imagem ou troque de ferramenta.",
        "تمت معايرة المقياس. اسحب لتحريك الصورة أو بدّل الأداة.",
    ),
    ("Espelhado", "معكوس"),
    ("Espessura", "السماكة"),
    ("Espessura da laje", "سماكة البلاطة"),
    ("Esquerda", "يسار"),
    (
        "Esta versão e o histórico dela serão removidos do projeto.",
        "سيُحذف هذا الإصدار وسجله من المشروع.",
    ),
    ("Estilo", "النمط"),
    ("Excluir", "حذف"),
    ("Excluir andar", "حذف الطابق"),
    ("Exibir", "إظهار"),
    ("Exportar 3D", "تصدير 3D"),
    ("Exportar planta", "تصدير المخطط"),
    ("Fechada", "مغلق"),
    ("Fechar", "إغلاق"),
    ("Fechar versão", "إغلاق الإصدار"),
    ("Fim", "النهاية"),
    ("Fim (x, y)", "النهاية (x, y)"),
    ("Grupo", "تجميع"),
    ("Hidráulica", "السباكة"),
    ("Hora do dia", "وقت اليوم"),
    ("Imagem", "صورة"),
    ("Imagem de fundo", "صورة الخلفية"),
    (
        "Imagem importada. Clique em dois pontos de medida conhecida para calibrar; arraste para posicionar.",
        "تم استيراد الصورة. انقر نقطتين بينهما مسافة معروفة للمعايرة؛ واسحب لتحريكها.",
    ),
    ("Imagem…", "صورة…"),
    ("Imagens", "الصور"),
    ("Importar imagem de fundo", "استيراد صورة خلفية"),
    ("Importar modelo 3D…", "استيراد نموذج 3D…"),
    ("Importar…", "استيراد…"),
    ("Informações", "معلومات"),
    ("Início", "البداية"),
    ("Início (x, y)", "البداية (x, y)"),
    ("Itálico", "مائل"),
    ("Lado direito", "الجهة اليمنى"),
    ("Lado esquerdo", "الجهة اليسرى"),
    ("Largura", "العرض"),
    (
        "Legenda de símbolos (elétrica e hidráulica)",
        "دليل الرموز (الكهرباء والسباكة)",
    ),
    ("Licença", "الترخيص"),
    ("Linhas (tubulação / eletroduto)", "خطوط (أنابيب / مواسير)"),
    ("Link", "رابط"),
    ("MCP desligado", "خادم MCP متوقف"),
    ("Manter proporções", "الحفاظ على النسب"),
    ("Marca", "العلامة التجارية"),
    ("Me paga um café", "ادعمني بفنجان قهوة"),
    ("Medida", "قياس"),
    ("Minha biblioteca", "مكتبتي"),
    ("Modelo", "النموذج"),
    (
        "Modelo da biblioteca posicionado. Arraste para o lugar.",
        "تم وضع نموذج المكتبة. اسحبه إلى مكانه.",
    ),
    (
        "Modelo importado. Ajuste medidas com Enter ou pelas alças.",
        "تم استيراد النموذج. عدّل المقاسات بـ Enter أو بالمقابض.",
    ),
    ("Modelos 3D", "نماذج 3D"),
    ("Modificar andar", "تعديل الطابق"),
    ("Modificar cota", "تعديل البُعد"),
    ("Modificar cômodo", "تعديل الغرفة"),
    ("Modificar linha", "تعديل الخط"),
    ("Modificar parede", "تعديل الجدار"),
    ("Modificar texto", "تعديل النص"),
    ("Modificar · Excluir · Cancelar", "تعديل · حذف · إلغاء"),
    ("Modificar…", "تعديل…"),
    ("Mostrar bússola", "إظهار البوصلة"),
    ("Move a seleção 1 cm (10 cm)", "يحرّك التحديد 1 سم (10 سم)"),
    ("Mover vista", "تحريك العرض"),
    ("Máxima", "الأفضل"),
    ("Móveis", "الأثاث"),
    ("Móveis na planta", "الأثاث في المخطط"),
    ("Nada encontrado.", "لم يُعثر على شيء."),
    ("Negrito", "عريض"),
    (
        "Nenhum espaço fechado por paredes aqui",
        "لا توجد مساحة محاطة بجدران هنا",
    ),
    ("Nenhum ponto", "لا توجد نقاط"),
    ("Nenhum ponto de vista salvo", "لا توجد زوايا عرض محفوظة"),
    ("Nome", "الاسم"),
    ("Nome do projeto", "اسم المشروع"),
    ("Norte", "الشمال"),
    ("Nova versão da planta", "إصدار جديد للمخطط"),
    ("Nova versão em branco", "إصدار فارغ جديد"),
    ("Novo", "جديد"),
    ("Novo (Ctrl+N)", "جديد (Ctrl+N)"),
    (
        "Níveis na mesma elevação funcionam como layouts alternativos",
        "المستويات على نفس المنسوب تعمل كتوزيعات بديلة",
    ),
    (
        "O andar e tudo o que está nele serão removidos (dá para desfazer).",
        "سيُحذف الطابق وكل ما عليه (يمكن التراجع).",
    ),
    (
        "O projeto tem alterações que ainda não foram salvas.",
        "المشروع فيه تغييرات غير محفوظة.",
    ),
    ("OBJ + MTL…", "OBJ + MTL…"),
    ("OK", "موافق"),
    ("Opacidade", "الشفافية"),
    ("Ordem (mesma elevação)", "الترتيب (نفس المنسوب)"),
    ("PDF (A3, ajustado à folha)…", "ملف PDF (A3، ملاءمة للورقة)…"),
    ("PDF 1:100…", "ملف PDF بمقياس 1:100…"),
    ("PDF 1:50…", "ملف PDF بمقياس 1:50…"),
    ("PNG…", "PNG…"),
    ("Paredes", "الجدران"),
    ("Inteiras", "أعلى"),
    ("Recortadas", "جدران مقطوعة"),
    ("Baixas", "أسفل"),
    ("Todas as paredes em pé", "كل الجدران قائمة"),
    (
        "As paredes da frente descem para mostrar os cômodos",
        "الجدران الأمامية تنخفض لإظهار الغرف",
    ),
    ("Todas as paredes descem", "كل الجدران تنخفض"),
    ("Personalizada", "مخصص"),
    ("Peça", "بلاط"),
    ("Pintura", "طلاء"),
    ("Piso", "الأرضية"),
    ("Planta", "المخطط"),
    ("Pontilhado", "منقّط"),
    ("Posição (x, y)", "الموضع (x, y)"),
    ("Potência da luz", "قوة الإضاءة"),
    ("Preço", "السعر"),
    ("Problemas", "المشكلات"),
    ("Profundidade", "العمق"),
    (
        "Projeto em edição: novos símbolos e linhas vão para ele",
        "المشروع قيد التحرير: الرموز والخطوط الجديدة تذهب إليه",
    ),
    (
        "Projetos (3D New Era AI, Sweet Home 3D)",
        "المشاريع (3D New Era AI، Sweet Home 3D)",
    ),
    ("Pé-direito", "ارتفاع السقف"),
    ("Qualidade", "الجودة"),
    ("Quantitativos", "الكميات"),
    ("Rascunho", "مسودة"),
    ("Recentes", "الأخيرة"),
    ("Recortar", "قص"),
    ("Refazer", "إعادة"),
    ("Refazer (Ctrl+Shift+Z)", "إعادة (Ctrl+Shift+Z)"),
    ("Referências dos cômodos", "مراجع الغرف"),
    ("Remover imagem", "إزالة الصورة"),
    ("Renderizar", "تصيير"),
    ("Renomear", "إعادة تسمية"),
    ("Rotação", "الدوران"),
    ("SVG em escala real…", "ملف SVG بالمقياس الحقيقي…"),
    ("Sair", "خروج"),
    ("Salvar", "حفظ"),
    ("Salvar (Ctrl+S)", "حفظ (Ctrl+S)"),
    ("Salvar PNG…", "حفظ PNG…"),
    ("Salvar alterações?", "حفظ التغييرات؟"),
    ("Salvar como…", "حفظ باسم…"),
    ("Salvar ponto de vista", "حفظ زاوية العرض"),
    ("Scroll · botão do meio · F", "العجلة · الزر الأوسط · F"),
    ("Selecionar", "تحديد"),
    ("Selecionar tudo", "تحديد الكل"),
    (
        "Selecionar · Mover vista · Paredes · Cômodos · Cotas · Texto",
        "تحديد · تحريك · جدران · غرف · أبعاد · نص",
    ),
    ("Sem acabamento", "بدون تشطيب"),
    ("Sem nome", "بدون اسم"),
    ("Sem seta", "بدون سهم"),
    ("Seta aberta", "سهم مفتوح"),
    ("Seta cheia", "سهم مصمت"),
    ("Setas (+Shift)", "أسهم (+Shift)"),
    ("Shift (segurado)", "مفتاح Shift (مضغوط)"),
    ("Sweet Home 3D", "Sweet Home 3D"),
    ("Símbolos arquitetônicos", "رموز معمارية"),
    ("Tamanho", "الحجم"),
    ("Teto", "السقف"),
    ("Texto", "نص"),
    ("Tipo", "النوع"),
    ("Tracejado", "متقطع"),
    ("Traço", "شَرطة"),
    ("Traço e dois pontos", "شَرطة نقطة نقطة"),
    ("Traço e ponto", "شَرطة نقطة"),
    ("Térreo", "الدور الأرضي"),
    ("Unidade", "الوحدة"),
    ("Unir paredes selecionadas", "وصل الجدران المحددة"),
    (
        "Usa o ponto de vista atual da vista 3D (aérea ou visitante).",
        "يستخدم زاوية العرض الحالية في 3D (جوي أو زائر).",
    ),
    ("Ver", "عرض"),
    ("Versão", "الإصدار"),
    ("Visitante", "زائر"),
    (
        "Visitante — arraste: olhar · W/A/S/D ou setas: andar · Scroll: avançar · Esc: visão aérea",
        "زائر — اسحب: النظر · W/A/S/D أو الأسهم: المشي · العجلة: التقدم · Esc: العرض الجوي",
    ),
    ("Vista de cima de todos", "منظر علوي للكل"),
    (
        "Visualização 3D requer o backend wgpu.",
        "العرض ثلاثي الأبعاد يحتاج إلى wgpu.",
    ),
    ("Visão aérea", "العرض الجوي"),
    ("Frente", "أمامي"),
    ("Trás", "خلفي"),
    ("Topo", "علوي"),
    ("Visível", "ظاهر"),
    ("Visível no 3D", "ظاهر في 3D"),
    ("Zoom · mover vista · enquadrar", "تكبير · تحريك · ملاءمة"),
    ("glTF binário (.glb)…", "ملف glTF ثنائي (.glb)…"),
    ("peças", "قطع"),
    ("Área", "المساحة"),
    ("Área na planta", "المساحة في المخطط"),
    ("Ângulo", "الزاوية"),
    ("Cotas", "الأبعاد"),
    ("Textos", "النصوص"),
    ("Linhas", "الخطوط"),
    ("paredes", "جدران"),
    ("cômodos", "غرف"),
    ("móveis", "قطع أثاث"),
    ("grupo de", "مجموعة من"),
    ("Sala de estar", "غرفة المعيشة"),
    ("Sala de jantar", "غرفة الطعام"),
    ("Cozinha", "المطبخ"),
    ("Quarto", "غرفة النوم"),
    ("Banheiro", "دورة المياه"),
    ("Lavanderia", "غرفة الغسيل"),
    ("Escritório", "المكتب"),
    ("Portas e janelas", "الأبواب والنوافذ"),
    ("Estrutura", "الهيكل"),
    ("Decoração", "الديكور"),
    ("Área externa", "خارجي"),
    ("atual", "الحالي"),
    (
        "Importado de {} — salve como projeto para manter tudo num arquivo",
        "مستورد من {} — احفظه كمشروع لتجمع كل شيء في ملف واحد",
    ),
    ("Aberto: {}", "تم الفتح: {}"),
    (" · {} aviso(s)", " · تحذيرات: {}"),
    ("⚠ Não foi possível abrir {}: {}", "⚠ تعذّر فتح {}: {}"),
    ("Pontos de vista ({})", "زوايا العرض ({})"),
    ("Ponto de vista {}", "زاوية العرض {}"),
    ("Salvo em {}", "حُفظ في {}"),
    ("⚠ Não foi possível salvar: {}", "⚠ تعذّر الحفظ: {}"),
    (
        "Modelo 3D exportado para {}",
        "صُدِّر النموذج ثلاثي الأبعاد إلى {}",
    ),
    ("⚠ Falha ao exportar: {}", "⚠ فشل التصدير: {}"),
    ("Planta exportada para {}", "صُدِّر المخطط إلى {}"),
    ("⚠ Imagem inválida: {}", "⚠ صورة غير صالحة: {}"),
    ("Modificar {} paredes", "تعديل {} جدران"),
    ("Modificar {}", "تعديل {}"),
    ("{} peças", "{} قطع"),
    (
        "Os pontos marcados estão a {} na escala atual.",
        "المسافة بين النقطتين المحددتين {} بالمقياس الحالي.",
    ),
    ("Fechar “{}”?", "إغلاق «{}»؟"),
    ("Excluir “{}”?", "حذف «{}»؟"),
    ("Renderizando… {} s", "جارٍ التصيير… {} ث"),
    ("Pronta em {} s", "جاهز خلال {} ث"),
    ("Foto salva em {}", "حُفظت الصورة في {}"),
    ("Mostrar {}", "إظهار {}"),
    ("Tema", "المظهر"),
    ("Do sistema", "حسب النظام"),
    ("Noite", "ليلي"),
    ("Dia", "نهاري"),
    ("luz", "إضاءة"),
    (
        "Arraste ou dois dedos: girar · Shift: mover · Pinça ou scroll: zoom · F: enquadrar",
        "اسحب أو بإصبعين: دوران · Shift: تحريك · قرص أو العجلة: تكبير · F: ملاءمة",
    ),
    ("Aparência", "المظهر"),
    ("Sofá 3 lugares", "كنبة 3 مقاعد"),
    ("Sofá 2 lugares", "كنبة مقعدين"),
    ("Sofá em L (chaise)", "كنبة على شكل L"),
    ("Poltrona", "كرسي بذراعين"),
    ("Poltrona Win", "كرسي Win بذراعين"),
    ("Cadeira Ares", "كرسي Ares"),
    ("Mesa de centro", "طاولة قهوة"),
    ("Mesa lateral", "طاولة جانبية"),
    ("Rack de TV", "وحدة تلفزيون"),
    ("Televisão 55\"", "تلفزيون 55 بوصة"),
    ("Estante", "وحدة رفوف"),
    ("Tapete", "سجادة"),
    ("Radiador de painel 60×10×50", "مشعاع حراري 60×10×50"),
    ("Radiador de painel 100×10×50", "مشعاع حراري 100×10×50"),
    ("Radiador de painel 120×10×50", "مشعاع حراري 120×10×50"),
    ("Convector elétrico 100×10×45", "مدفأة كهربائية 100×10×45"),
    ("Luminária de piso", "مصباح أرضي"),
    ("Abajur de mesa", "مصباح طاولة"),
    ("Spot embutido LED 7 W", "سبوت LED مدمج 7 واط"),
    ("Pendente", "إضاءة معلّقة"),
    ("Painel LED 60×60 36 W", "لوح LED 60×60 بقوة 36 واط"),
    ("Fita LED (perfil)", "شريط LED (بروفايل)"),
    ("Mesa de jantar 4 lugares", "طاولة طعام لـ 4"),
    ("Mesa de jantar 6 lugares", "طاولة طعام لـ 6"),
    ("Mesa com 4 cadeiras", "طاولة مع 4 كراسي"),
    ("Mesa com 6 cadeiras", "طاولة مع 6 كراسي"),
    ("Mesa redonda", "طاولة دائرية"),
    ("Cadeira", "كرسي"),
    ("Aparador", "بوفيه"),
    ("Geladeira", "ثلاجة"),
    ("Fogão 4 bocas", "طباخ 4 عيون"),
    ("Bancada com pia", "سطح عمل مع مغسلة"),
    ("Armário de cozinha (baixo)", "خزانة سفلية"),
    ("Armário aéreo", "خزانة علوية"),
    ("Coifa", "شفاط"),
    ("Lava-louças", "غسالة صحون"),
    ("Micro-ondas", "مايكروويف"),
    ("Cooktop 4 bocas (embutir)", "موقد 4 عيون (مدمج)"),
    ("Cuba de inox (embutir)", "مغسلة ستانلس ستيل (مدمجة)"),
    ("Forno de embutir", "فرن مدمج"),
    ("Ilha de cozinha", "جزيرة مطبخ"),
    ("Banqueta alta", "كرسي بار"),
    ("Cama de casal", "سرير مزدوج"),
    ("Cama queen", "سرير كوين"),
    ("Cama king", "سرير كينج"),
    ("Cama de solteiro", "سرير مفرد"),
    ("Criado-mudo", "كومودينو"),
    ("Guarda-roupa", "دولاب ملابس"),
    ("Cômoda", "تسريحة أدراج"),
    ("Berço", "سرير أطفال"),
    ("Vaso sanitário", "مرحاض"),
    ("Gabinete com lavatório", "وحدة مغسلة مع حوض"),
    ("Box de chuveiro", "قاعدة دش"),
    ("Box de vidro", "حاجز دش زجاجي"),
    ("Banheira", "بانيو"),
    ("Máquina de lavar", "غسالة ملابس"),
    ("Secadora", "نشافة ملابس"),
    ("Tanque", "حوض غسيل"),
    ("Escrivaninha", "مكتب"),
    ("Cadeira de escritório", "كرسي مكتب"),
    ("Porta", "باب"),
    ("Porta dupla", "باب مزدوج"),
    ("Porta de correr", "باب منزلق"),
    ("Portão de garagem", "باب كراج"),
    ("Vão livre", "فتحة"),
    ("Janela", "نافذة"),
    ("Janela basculante", "نافذة علوية مفصلية"),
    ("Porta-janela", "باب فرنسي"),
    ("Escada reta", "درج مستقيم"),
    ("Pilar", "عمود"),
    ("Coluna redonda", "عمود دائري"),
    ("Sapata", "قاعدة خرسانية"),
    ("Viga / caibro", "جسر / عارضة"),
    ("Painel", "لوح"),
    ("Telha ondulada", "صاج مموج"),
    (
        "Gradil de ferro (guarda-corpo de barras verticais)",
        "درابزين حديد (قضبان رأسية)",
    ),
    (
        "Guarda-corpo de vidro laminado com corrimão metálico",
        "حاجز زجاج مقسّى مع درابزين معدني",
    ),
    (
        "Fechamento de vidro da sacada (envidraçamento retrátil)",
        "زجاج شرفة (قابل للطي)",
    ),
    ("Caixa", "صندوق"),
    ("Planta em vaso", "نبتة في أصيص"),
    ("Floreira", "حوض زراعة"),
    ("Muro / cerca", "جدار / سور"),
    ("Piscina", "مسبح"),
    ("Piscina oval", "مسبح بيضاوي"),
    ("Espreguiçadeira", "كرسي استلقاء"),
    ("Churrasqueira", "شواية"),
    ("Banco", "مقعد"),
    ("Árvore", "شجرة"),
    ("Carro", "سيارة"),
    ("Tomada baixa (30 cm)", "مقبس منخفض (30 سم)"),
    ("Tomada média (1,10 m)", "مقبس متوسط (1.10 م)"),
    ("Tomada alta (2,20 m)", "مقبس مرتفع (2.20 م)"),
    ("Interruptor simples", "مفتاح مفرد"),
    ("Interruptor duplo", "مفتاح مزدوج"),
    ("Interruptor paralelo (three-way)", "مفتاح تحويلة (اتجاهين)"),
    ("Ponto de luz no teto", "نقطة إنارة سقفية"),
    (
        "Arandela (ponto de luz na parede)",
        "إنارة جدارية (نقطة إنارة على الجدار)",
    ),
    ("Quadro de distribuição", "لوحة التوزيع"),
    ("Ponto de ar-condicionado", "نقطة مكيف"),
    ("Ponto para chuveiro elétrico", "نقطة سخان دش كهربائي"),
    ("Tomada de dados / TV", "مقبس بيانات / تلفزيون"),
    ("Ponto de rede RJ45 (Cat 6)", "نقطة شبكة RJ45 (Cat 6)"),
    ("Ponto de TV (coaxial)", "نقطة تلفزيون (محوري)"),
    (
        "Ponto de Wi-Fi no teto (access point)",
        "نقطة Wi-Fi سقفية (Access Point)",
    ),
    (
        "Quadro de telecomunicações (rack / DG)",
        "لوحة الاتصالات (Rack / MDF)",
    ),
    ("Campainha", "جرس الباب"),
    (
        "Torre de tomadas retrátil de embutir (furo 60 mm, 3 tomadas, plugue)",
        "برج مقابس منزلق مدمج (فتحة 60 مم، 3 مقابس، قابس)",
    ),
    (
        "Torre de tomadas automática com indução (furo 85 mm, ligada à instalação)",
        "برج مقابس أوتوماتيكي مع شحن لاسلكي (فتحة 85 مم، توصيل ثابت)",
    ),
    (
        "Torre de 4 tomadas + USB e indução (furo 100 mm)",
        "برج 4 مقابس + USB وشحن لاسلكي (فتحة 100 مم)",
    ),
    (
        "Caixa de tomadas de mesa (4 tomadas, RJ45, USB, HDMI; recorte 120×335 mm)",
        "علبة مقابس مكتبية (4 مقابس، RJ45، USB، HDMI؛ قطع 120×335 مم)",
    ),
    (
        "Tomada de embutir em móvel (furo 35 mm)",
        "مقبس مدمج في الأثاث (فتحة 35 مم)",
    ),
    (
        "Relé de automação (atrás do interruptor ou da luminária)",
        "ريليه أتمتة (خلف المفتاح أو وحدة الإنارة)",
    ),
    ("Interruptor inteligente", "مفتاح ذكي"),
    ("Dimmer", "معتّم إضاءة (Dimmer)"),
    ("Sensor de presença de teto", "حساس حركة سقفي"),
    ("Fechadura eletrônica", "قفل إلكتروني"),
    ("Ponto de água fria", "نقطة ماء بارد"),
    ("Ponto de água quente", "نقطة ماء حار"),
    ("Ponto de esgoto", "نقطة صرف"),
    (
        "Caixa sifonada 150×150×50 (ralo)",
        "صفاية بسيفون 150×150×50 (صرف)",
    ),
    (
        "Caixa sifonada 100×150×50 (ralo)",
        "صفاية بسيفون 100×150×50 (صرف)",
    ),
    (
        "Caixa sifonada 150×185×75 (ralo)",
        "صفاية بسيفون 150×185×75 (صرف)",
    ),
    (
        "Ralo sifonado 100 mm (fecho hídrico curto)",
        "صفاية أرضية بسيفون 100 مم (حاجز ضحل)",
    ),
    ("Ralo seco 100 mm", "صفاية أرضية جافة 100 مم"),
    (
        "Ralo linear 70 cm (sem sifão)",
        "صفاية خطية 70 سم (بدون سيفون)",
    ),
    ("Ralo linear sifonado 70 cm", "صفاية خطية بسيفون 70 سم"),
    (
        "Ralo de águas pluviais (varanda descoberta, terraço)",
        "تصريف مياه الأمطار (شرفة مفتوحة، سطح)",
    ),
    ("Registro", "محبس"),
    ("Caixa de gordura", "مصيدة شحوم"),
    (
        "Grelha de ventilação permanente (gás)",
        "فتحة تهوية دائمة (غاز)",
    ),
    ("Tubo de ventilação", "ماسورة تهوية"),
    ("Caixa de inspeção", "غرفة تفتيش"),
    ("Hidrômetro", "عداد المياه"),
    ("Ponto de gás", "نقطة غاز"),
    ("Drywall 73 mm", "جبس بورد 73 مم"),
    ("Drywall 95 mm", "جبس بورد 95 مم"),
    ("Drywall 115 mm", "جبس بورد 115 مم"),
    ("Drywall 120 mm chapa dupla", "جبس بورد 120 مم، لوح مزدوج"),
    ("Tijolo cerâmico 9 cm", "طوب طيني 9 سم"),
    ("Tijolo cerâmico 14 cm", "طوب طيني 14 سم"),
    ("Tijolo cerâmico 19 cm", "طوب طيني 19 سم"),
    ("Bloco de concreto 14 cm", "بلوك خرساني 14 سم"),
    ("Bloco de concreto 19 cm", "بلوك خرساني 19 سم"),
    ("Parede de concreto 10 cm", "جدار خرساني 10 سم"),
    ("Concreto armado 15 cm", "خرسانة مسلحة 15 سم"),
    ("Steel frame 14 cm", "هيكل فولاذي 14 سم"),
    ("Wood frame 10 cm", "هيكل خشبي 10 سم"),
    ("Divisória de vidro", "قاطع زجاجي"),
    (
        "Montante 48 mm + 1 chapa de 12,5 mm em cada face",
        "قائم 48 مم + لوح 12.5 مم من كل جهة",
    ),
    (
        "Montante 70 mm + 1 chapa de 12,5 mm em cada face",
        "قائم 70 مم + لوح 12.5 مم من كل جهة",
    ),
    (
        "Montante 90 mm + 1 chapa de 12,5 mm em cada face",
        "قائم 90 مم + لوح 12.5 مم من كل جهة",
    ),
    (
        "Montante 70 mm + 2 chapas de 12,5 mm em cada face (acústica)",
        "قائم 70 مم + لوحان 12.5 مم من كل جهة (عازل للصوت)",
    ),
    (
        "Bloco de 9 cm + reboco de 2,5 cm em cada face",
        "بلوك 9 سم + لياسة 2.5 سم من كل جهة",
    ),
    (
        "Bloco de 14 cm + reboco de 2,5 cm em cada face",
        "بلوك 14 سم + لياسة 2.5 سم من كل جهة",
    ),
    (
        "Bloco de 19 cm + reboco de 2,5 cm em cada face",
        "بلوك 19 سم + لياسة 2.5 سم من كل جهة",
    ),
    (
        "Bloco estrutural de 14 cm + revestimento de 1,5 cm em cada face",
        "بلوك إنشائي 14 سم + تشطيب 1.5 سم من كل جهة",
    ),
    (
        "Bloco estrutural de 19 cm + revestimento de 1,5 cm em cada face",
        "بلوك إنشائي 19 سم + تشطيب 1.5 سم من كل جهة",
    ),
    (
        "Concreto moldado in loco, sistema parede de concreto",
        "خرسانة مصبوبة في الموقع، نظام جدران خرسانية",
    ),
    (
        "Parede estrutural de concreto armado",
        "جدار خرسانة مسلحة إنشائي",
    ),
    (
        "Perfil 90 mm + OSB e placa cimentícia/gesso",
        "بروفايل 90 مم + OSB ولوح أسمنتي/جبسي",
    ),
    (
        "Estrutura de madeira com fechamento em chapas",
        "هيكل خشبي مغلق بالألواح",
    ),
    ("Vidro temperado de 10 mm", "زجاج مقسّى 10 مم"),
    ("Madeira (réguas)", "خشب (ألواح)"),
    ("Taco espinha de peixe", "باركيه متعرج"),
    ("Porcelanato / cerâmica", "بورسلان / سيراميك"),
    ("Azulejo metrô", "بلاط سبواي"),
    ("Tijolo aparente", "طوب ظاهر"),
    ("Pedra", "حجر"),
    ("Cimento queimado", "أسمنت مصقول"),
    ("Mármore", "رخام"),
    ("Carpete", "موكيت"),
    ("Grama", "عشب"),
    ("Água (piscina)", "ماء (مسبح)"),
    ("Deck de madeira", "أرضية خشبية خارجية"),
    (
        "Projeto recuperado desta sessão.",
        "تم استرجاع المشروع من هذه الجلسة.",
    ),
    ("No terminal:", "في الطرفية:"),
    (
        "Os instaladores já registram sozinhos se o Claude Code estiver instalado.",
        "برامج التثبيت تسجّله لك عند وجود Claude Code.",
    ),
    (
        "Configurações → Desenvolvedor → Editar configuração",
        "Settings → Developer → Edit Config",
    ),
    (
        "Precisa do Node.js instalado para a ponte mcp-remote. Reinicie o Claude Desktop.",
        "يحتاج Node.js لجسر mcp-remote. أعد تشغيل Claude Desktop.",
    ),
    (
        "Os instaladores já registram sozinhos se o Codex estiver instalado.",
        "برامج التثبيت تسجّله لك عند وجود Codex.",
    ),
    (
        "Depois abra o Gemini CLI e peça o projeto.",
        "ثم افتح Gemini CLI واطلب مشروعك.",
    ),
    (
        "Use no modo agente do Copilot.",
        "استخدمه في وضع الوكيل في Copilot.",
    ),
    ("Arquivo ~/.cursor/mcp.json", "الملف ~/.cursor/mcp.json"),
    (
        "Reinicie o Cursor depois de salvar.",
        "أعد تشغيل Cursor بعد الحفظ.",
    ),
    ("Outro app", "تطبيق آخر"),
    (
        "Nas configurações de MCP do app (Cline, Roo Code, Cherry Studio, LM Studio…)",
        "في إعدادات MCP للتطبيق (Cline، Roo Code، Cherry Studio، LM Studio…)",
    ),
    (
        "O que importa é o app falar MCP: o modelo pode ser qualquer um.",
        "المهم أن يدعم التطبيق MCP: النموذج يمكن أن يكون أيًّا منها.",
    ),
    ("agora", "الآن"),
    ("há {} s", "قبل {} ث"),
    ("há {} min", "قبل {} د"),
    ("há {} h", "قبل {} س"),
    ("{} · {} chamadas", "الاتصال: {} · {} استدعاء"),
    (
        "MCP · esperando sua IA",
        "خادم MCP · بانتظار الذكاء الاصطناعي",
    ),
    ("IA: só no aplicativo", "الذكاء الاصطناعي: في التطبيق فقط"),
    (
        "Conectar sua IA — clique para ver como",
        "اربط الذكاء الاصطناعي — انقر لمعرفة الطريقة",
    ),
    ("IA", "الذكاء الاصطناعي"),
    ("{} conectado · {}", "متصل: {} · {}"),
    ("Nenhuma IA conectada ainda", "لا يوجد ذكاء اصطناعي متصل بعد"),
    ("Conectar sua IA…", "اربط الذكاء الاصطناعي…"),
    ("Copiar o endereço do MCP", "نسخ عنوان MCP"),
    ("Endereço copiado", "تم نسخ العنوان"),
    ("Conectar sua IA", "اربط الذكاء الاصطناعي"),
    (
        "O editor abre uma porta MCP: a sua IA desenha aqui dentro, em centímetros, e você vê acontecer.",
        "يفتح المحرر منفذ MCP: يرسم الذكاء الاصطناعي هنا بالسنتيمتر، وأنت تشاهد.",
    ),
    ("Agora", "الآن"),
    ("Servidor MCP ligado", "خادم MCP يعمل"),
    (
        "No navegador o editor roda sozinho: o MCP vive no aplicativo do computador.",
        "في المتصفح يعمل المحرر وحده: خادم MCP موجود في تطبيق سطح المكتب.",
    ),
    (
        "O servidor está desligado (--no-server): reabra o aplicativo sem essa opção.",
        "الخادم متوقف (--no-server): أعد فتح التطبيق بدون هذا الخيار.",
    ),
    (
        "Nenhuma IA conectada ainda — siga os três passos abaixo.",
        "لا يوجد ذكاء اصطناعي متصل بعد — الخطوات الثلاث أدناه تكفي.",
    ),
    (
        "1. Escolha o aplicativo de IA que você usa",
        "1. اختر تطبيق الذكاء الاصطناعي الذي تستخدمه",
    ),
    ("2. Cole isto onde ele pede", "2. الصق هذا حيث يُطلب"),
    ("3. Peça alguma coisa", "3. اطلب منه شيئًا"),
    (
        "Quantas paredes tem este projeto? Depois coloque uma janela de 120 cm na sala.",
        "كم جدارًا في هذا المشروع؟ ثم ضع نافذة بعرض 120 سم في غرفة المعيشة.",
    ),
    ("Copiado", "تم النسخ"),
    ("Últimas chamadas", "آخر الاستدعاءات"),
    (
        "Nada ainda. Assim que sua IA usar uma ferramenta, ela aparece aqui.",
        "لا شيء بعد. بمجرد أن يستخدم الذكاء الاصطناعي أداة، ستظهر هنا.",
    ),
    ("Baixar o aplicativo", "تنزيل التطبيق"),
    (
        "{} conectado · {} chamadas · {}",
        "متصل: {} · {} استدعاء · {}",
    ),
    (
        "{} conectou — sua IA já pode desenhar aqui",
        "تم الاتصال بـ {} — يستطيع الذكاء الاصطناعي الرسم هنا الآن",
    ),
    ("Registrar agora", "سجّل الآن"),
    ("Registrando…", "جارٍ التسجيل…"),
    (
        "Roda esse mesmo comando aqui, sem abrir o terminal.",
        "ينفّذ الأمر نفسه من هنا، بدون طرفية.",
    ),
    (
        "Registrado no {} — agora é só pedir",
        "تم التسجيل في {} — اطلب ما تريد الآن",
    ),
    ("Não deu para registrar: {}", "تعذّر التسجيل: {}"),
    (
        "Ligue e esta aba ganha um endereço que a sua IA alcança. O projeto não sai daqui: o servidor no meio só passa recados.",
        "فعّله لتحصل هذه الصفحة على عنوان يصل إليه الذكاء الاصطناعي. المشروع يبقى هنا: الخادم الوسيط ينقل الرسائل فقط.",
    ),
    ("Ligar o MCP nesta aba", "تفعيل MCP في هذه الصفحة"),
    ("Ligando…", "جارٍ التفعيل…"),
    (
        "Esta aba está no ar para a sua IA",
        "هذه الصفحة متاحة للذكاء الاصطناعي",
    ),
    (
        "Quem tiver o endereço abaixo pode editar este projeto: trate como senha.",
        "أي شخص لديه العنوان أدناه يستطيع تعديل هذا المشروع: عامله ككلمة مرور.",
    ),
    ("Não deu para ligar: {}", "تعذّر التفعيل: {}"),
    ("Tentar de novo", "حاول مرة أخرى"),
    (
        "{} só no aplicativo do computador: aqui ele renderiza na CPU e travaria a aba",
        "{} في تطبيق سطح المكتب فقط: هنا يُعالج على المعالج المركزي وسيجمّد الصفحة",
    ),
    ("MCP desligado · ligar", "خادم MCP متوقف · فعّله"),
    ("Desligar", "إيقاف"),
    (
        "o relay recusou esta aba — ligue de novo",
        "رفض الوسيط هذه الصفحة — فعّله مرة أخرى",
    ),
    ("Conectado como {}", "مسجّل الدخول باسم {}"),
    (
        "Com este endereço, o Claude, o ChatGPT e outras IAs chegam nesta aba entrando com a sua conta.",
        "بهذا العنوان يصل Claude و ChatGPT وغيرهما إلى هذه الصفحة بتسجيل الدخول بحسابك.",
    ),
    (
        "Entrar para usar no Claude e no ChatGPT",
        "سجّل الدخول لاستخدامه في Claude و ChatGPT",
    ),
    (
        "Com uma conta, a sua IA chega nesta aba por um endereço fixo, sem colar nada.",
        "مع الحساب، يصل الذكاء الاصطناعي إلى هذه الصفحة بعنوان ثابت، بدون لصق أي شيء.",
    ),
    (
        "Fecha o endereço. A sua IA perde o acesso na hora; o projeto continua aqui.",
        "يغلق العنوان. يفقد الذكاء الاصطناعي الوصول فورًا؛ والمشروع يبقى هنا.",
    ),
    (
        "Conectar sua IA (no navegador, liga e desliga)",
        "اربط الذكاء الاصطناعي (في المتصفح، للتفعيل والإيقاف)",
    ),
    ("3D", "3D"),
    (
        "Catálogo · Planta · 3D (telas estreitas)",
        "الكتالوج · المخطط · 3D (الشاشات الضيقة)",
    ),
    (
        "o servidor que leva a sua IA até esta aba não respondeu — no aplicativo do computador o MCP não precisa dele",
        "الخادم الذي يوصل الذكاء الاصطناعي إلى هذه الصفحة لم يستجب — في تطبيق سطح المكتب لا يحتاج MCP إلى خادم",
    ),
    ("Disciplina, camadas e andares", "التخصص والطبقات والطوابق"),
    ("IA · esperando", "الذكاء الاصطناعي · بالانتظار"),
    ("Conectar IA", "اربط الذكاء الاصطناعي"),
    ("{} conectado · {} chamadas", "متصل: {} · {} استدعاء"),
    ("Esperando a sua IA chegar", "بانتظار وصول الذكاء الاصطناعي"),
    (
        "Ainda não dá para alcançar esta janela",
        "لا شيء يستطيع الوصول إلى هذه النافذة بعد",
    ),
    (
        "Escolha o aplicativo de IA que você usa",
        "اختر تطبيق الذكاء الاصطناعي الذي تستخدمه",
    ),
    ("Cole isto onde ele pede", "الصق هذا حيث يُطلب"),
    ("Peça alguma coisa", "اطلب منه شيئًا"),
    (
        "o endereço guardado não vale mais — ligue de novo",
        "العنوان المحفوظ هنا لم يعد لك — فعّله مرة أخرى",
    ),
    ("No ar, esperando a sua IA", "يعمل، بانتظار الذكاء الاصطناعي"),
    (
        "Quem tiver esse endereço edita este projeto: trate como senha.",
        "أي شخص لديه هذا العنوان يعدّل هذا المشروع: عامله ككلمة مرور.",
    ),
    (
        "⚠ Importe primeiro a imagem da planta (Imagem de fundo → Importar…).",
        "⚠ استورد صورة المخطط أولًا (صورة الخلفية ← استيراد…).",
    ),
    (
        "⚠ Nenhuma parede encontrada. Confira a escala ou tente o outro modo de conversão.",
        "⚠ لم يُعثر على جدران. تحقق من المقياس أو جرّب طريقة التحويل الأخرى.",
    ),
    (
        "{} paredes criadas a partir da planta. Veja a vista 3D; Ctrl+Z desfaz.",
        "تم إنشاء {} جدار من المخطط. شاهد العرض ثلاثي الأبعاد؛ Ctrl+Z للتراجع.",
    ),
    ("Converter planta em 3D", "تحويل المخطط إلى 3D"),
    (
        "Planta impressa ou exportada, com paredes preenchidas. Calibre a escala antes.",
        "مخطط مطبوع أو مُصدَّر بجدران مُعبّأة. عاير المقياس أولًا.",
    ),
    ("Converter croqui em 3D", "تحويل الكروكي إلى 3D"),
    (
        "Desenho à mão (croqui) com linhas finas. Calibre a escala antes.",
        "رسم يدوي (كروكي) بخطوط رفيعة. عاير المقياس أولًا.",
    ),
];

/// The Arabic for a Portuguese text of the interface.
pub(super) fn arabic(pt: &str) -> Option<&'static str> {
    static MAP: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    MAP.get_or_init(|| TABLE.iter().copied().collect())
        .get(pt)
        .copied()
}

/// Whether `c` is written in Arabic script.
fn is_arabic(c: char) -> bool {
    matches!(c, '\u{0600}'..='\u{06FF}' | '\u{0750}'..='\u{077F}' | '\u{08A0}'..='\u{08FF}'
        | '\u{FB50}'..='\u{FDFF}' | '\u{FE70}'..='\u{FEFF}')
}

/// A letter or digit that reads left to right inside Arabic text: a key
/// name, a number, a file name.
fn is_ltr(c: char) -> bool {
    c.is_alphanumeric() && !is_arabic(c)
}

/// Longest line before a long Arabic text is broken: egui wraps an Arabic
/// run after shaping it, which would put the end of a sentence on its first
/// row, so long texts are broken here instead, a line per paragraph.
const LINE: usize = 52;

/// Text as egui should be given it. Arabic needs two things egui does not do:
///
/// - Within a run the shaper lays everything out right to left, numbers and
///   Latin words too, so each left-to-right stretch is reversed beforehand
///   to come out the right way round.
/// - Long sentences are broken into lines (see [`LINE`]).
///
/// Text with no Arabic in it comes back as it is.
pub(super) fn visual(text: &str) -> String {
    if !text.chars().any(is_arabic) {
        return text.to_owned();
    }
    text.split('\n')
        .flat_map(wrap)
        .map(|line| reverse_ltr_runs(&line))
        .collect::<Vec<_>>()
        .join("\n")
}

/// `line` broken at spaces into lines of at most [`LINE`] characters, when it
/// is clearly longer than that.
fn wrap(line: &str) -> Vec<String> {
    if line.chars().count() <= LINE + LINE / 4 {
        return vec![line.to_owned()];
    }
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in line.split(' ') {
        // A line takes its direction from its first letter, so a new line
        // only ever starts on an Arabic word.
        if !current.is_empty()
            && current.chars().count() + 1 + word.chars().count() > LINE
            && starts_arabic(word)
        {
            lines.push(std::mem::take(&mut current));
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }
    lines.push(current);
    lines
}

/// Whether the first letter of `text` is Arabic. egui's shaper takes the
/// direction of a run from it: a line that starts with a Latin word is laid
/// out left to right, Arabic and all.
fn starts_arabic(text: &str) -> bool {
    text.chars()
        .find(|c| c.is_alphabetic())
        .is_some_and(is_arabic)
}

/// Every stretch from one left-to-right character to the last one before
/// Arabic starts again, reversed.
fn reverse_ltr_runs(line: &str) -> String {
    let chars: Vec<char> = line.chars().collect();
    let mut out = Vec::with_capacity(chars.len());
    let mut i = 0;
    while i < chars.len() {
        if !is_ltr(chars[i]) {
            out.push(chars[i]);
            i += 1;
            continue;
        }
        // The run ends at its last left-to-right character before Arabic.
        let mut end = i;
        let mut j = i;
        while j < chars.len() && !is_arabic(chars[j]) {
            if is_ltr(chars[j]) {
                end = j;
            }
            j += 1;
        }
        out.extend(chars[i..=end].iter().rev());
        i = end + 1;
    }
    out.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_without_arabic_is_left_alone() {
        assert_eq!(visual("Ctrl+Z (copy) 12.5"), "Ctrl+Z (copy) 12.5");
    }

    #[test]
    fn numbers_and_latin_inside_arabic_are_reversed_for_the_shaper() {
        assert_eq!(visual("حفظ (Ctrl+S)"), "حفظ (S+lrtC)");
        assert_eq!(visual("قبل 15 ث"), "قبل 51 ث");
        assert_eq!(visual("تم الفتح: casa.n3d"), "تم الفتح: d3n.asac");
        // Spaces between two Latin words stay inside the run.
        assert_eq!(visual("ادعم عبر Buy Me"), "ادعم عبر eM yuB");
    }

    #[test]
    fn long_sentences_are_broken_into_lines() {
        let long = arabic(
            "Comece desenhando paredes (W), importe uma planta como imagem de fundo, ou peça para a IA via MCP.",
        )
        .unwrap();
        let shown = visual(long);
        assert!(shown.lines().count() > 1);
        assert!(shown.lines().all(|l| l.chars().count() <= LINE));
        // Nothing lost but the spaces the breaks replaced.
        assert_eq!(
            shown
                .replace('\n', " ")
                .chars()
                .filter(|c| is_arabic(*c))
                .count(),
            long.chars().filter(|c| is_arabic(*c)).count()
        );
    }

    /// Every piece in the catalog has an Arabic name.
    #[test]
    fn every_piece_of_the_catalog_has_an_arabic_name() {
        let missing: Vec<&str> = newera_catalog::CATALOG
            .iter()
            .map(|item| item.name)
            .filter(|name| arabic(name).is_none())
            .collect();
        assert!(missing.is_empty(), "no Arabic: {missing:#?}");
    }

    /// Every Arabic text answers a text the other languages translate too:
    /// none is left behind when the Portuguese changes.
    #[test]
    fn every_arabic_text_has_a_portuguese_original() {
        let stale: Vec<&str> = TABLE
            .iter()
            .map(|(pt, _)| *pt)
            .filter(|pt| !super::super::has_translation(pt))
            .collect();
        assert!(stale.is_empty(), "{stale:#?}");
        assert_eq!(TABLE.len(), arabic_count(), "a text is listed twice");
    }

    fn arabic_count() -> usize {
        TABLE
            .iter()
            .map(|(pt, _)| *pt)
            .collect::<std::collections::HashSet<_>>()
            .len()
    }

    /// Every Arabic text starts with an Arabic word (or a number, which
    /// sets no direction), never a Latin one or a name filled in.
    #[test]
    fn every_arabic_text_starts_right_to_left() {
        let wrong: Vec<&str> = TABLE
            .iter()
            .map(|(_, ar)| *ar)
            .filter(|ar| ar.chars().any(is_arabic))
            .filter(|ar| {
                let head = ar.split("{}").next().unwrap_or_default();
                !starts_arabic(head) && !ar.starts_with("{} ")
                    || ar.starts_with("{} ") && !ar[3..].starts_with(|c: char| is_arabic(c))
            })
            .collect();
        assert!(wrong.is_empty(), "{wrong:#?}");
        for (_, ar) in TABLE {
            for line in visual(ar).lines().filter(|l| l.chars().any(is_arabic)) {
                assert!(starts_arabic(line) || line.starts_with(" · "), "{line}");
            }
        }
    }

    /// The placeholders a sentence carries survive the translation.
    #[test]
    fn translations_keep_their_placeholders() {
        for (pt, ar) in TABLE {
            assert_eq!(
                pt.matches("{}").count(),
                ar.matches("{}").count(),
                "{pt} → {ar}"
            );
        }
    }
}
