#include <vector>

using namespace::std;

/////////////////////////////////////////
//CWord
/////////////////////////////////////////
class CWord
{
public:
	int m_nID;
	char* m_psWord;
public:
	CWord();
	~CWord();
};

CWord::CWord()
{
	m_psWord = NULL;
	m_nID = -1;
}

CWord::~CWord()
{
}
